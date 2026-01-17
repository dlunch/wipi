#![cfg_attr(target_os = "none", no_std)]
#![cfg(not(target_os = "none"))]

pub mod graphics;
pub mod kernel;

use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::Surface;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use graphics::{SCREEN_HEIGHT, SCREEN_WIDTH};

struct SimulationApp {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    paint_clet: unsafe extern "C" fn(),
    handle_input: unsafe extern "C" fn(i32, i32, i32),
}

impl SimulationApp {
    fn new(
        paint_clet: unsafe extern "C" fn(),
        handle_input: unsafe extern "C" fn(i32, i32, i32),
    ) -> Self {
        Self {
            window: None,
            surface: None,
            paint_clet,
            handle_input,
        }
    }

    fn render(&mut self) {
        let Some(surface) = &mut self.surface else {
            return;
        };
        let Some(window) = &self.window else {
            return;
        };

        unsafe { (self.paint_clet)() };

        let screen = graphics::SCREEN_FRAMEBUFFER.lock().unwrap();
        let buffer = screen.buffer();

        let width = SCREEN_WIDTH;
        let height = SCREEN_HEIGHT;

        let mut sb_buffer = surface.buffer_mut().unwrap();
        for y in 0..height {
            for x in 0..width {
                let offset = y * (SCREEN_WIDTH * 4) + x * 4;
                if offset + 3 < buffer.len() {
                    let b = buffer[offset] as u32;
                    let g = buffer[offset + 1] as u32;
                    let r = buffer[offset + 2] as u32;
                    sb_buffer[y * width + x] = (r << 16) | (g << 8) | b;
                }
            }
        }
        sb_buffer.present().unwrap();
        window.request_redraw();
    }
}

impl ApplicationHandler for SimulationApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes()
            .with_title("WIPI Simulation")
            .with_inner_size(PhysicalSize::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32))
            .with_resizable(false);

        let window = Rc::new(event_loop.create_window(window_attrs).unwrap());

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let mut surface = Surface::new(&context, window.clone()).unwrap();

        surface
            .resize(
                NonZeroU32::new(SCREEN_WIDTH as u32).unwrap(),
                NonZeroU32::new(SCREEN_HEIGHT as u32).unwrap(),
            )
            .unwrap();

        self.window = Some(window.clone());
        self.surface = Some(surface);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed()
                    && let winit::keyboard::PhysicalKey::Code(key_code) = event.physical_key
                {
                    let key = match key_code {
                        winit::keyboard::KeyCode::ArrowUp => -1,
                        winit::keyboard::KeyCode::ArrowDown => -2,
                        winit::keyboard::KeyCode::ArrowLeft => -3,
                        winit::keyboard::KeyCode::ArrowRight => -4,
                        winit::keyboard::KeyCode::Space => -5,
                        _ => 0,
                    };
                    if key != 0 {
                        unsafe { (self.handle_input)(1, key, 0) };
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn simulation_start(
    start_clet: unsafe extern "C" fn(),
    paint_clet: unsafe extern "C" fn(),
    handle_input: unsafe extern "C" fn(i32, i32, i32),
) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        kernel::set_resource_path(std::path::PathBuf::from(&args[1]));
    }

    unsafe { start_clet() };

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = SimulationApp::new(paint_clet, handle_input);
    event_loop.run_app(&mut app).unwrap();
}
