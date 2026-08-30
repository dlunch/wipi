#![cfg_attr(target_os = "none", no_std)]
#![cfg(not(target_os = "none"))]

pub mod database;
pub mod graphics;
pub mod kernel;
pub mod media;

use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::Surface;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use graphics::{SCREEN_HEIGHT, SCREEN_WIDTH};

struct SimulationApp {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    paint_clet: unsafe extern "C" fn(),
    handle_input: unsafe extern "C" fn(i32, i32, i32),
    surface_width: usize,
    surface_height: usize,
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
            surface_width: SCREEN_WIDTH,
            surface_height: SCREEN_HEIGHT,
        }
    }

    fn render(&mut self) {
        let Some(surface) = &mut self.surface else {
            return;
        };

        unsafe { (self.paint_clet)() };

        let screen = graphics::SCREEN_FRAMEBUFFER.lock().unwrap();
        let buffer = screen.buffer();

        let width = self.surface_width;
        let height = self.surface_height;

        let mut sb_buffer = surface.buffer_mut().unwrap();
        blit_bilinear(buffer, &mut sb_buffer, width, height);
        sb_buffer.present().unwrap();
    }

    fn resize_surface(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        let Some(surface) = &mut self.surface else {
            return;
        };

        surface
            .resize(
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            )
            .unwrap();

        self.surface_width = width as usize;
        self.surface_height = height as usize;
    }

    fn scaled_logical_size(dpi_scale: f64) -> LogicalSize<f64> {
        LogicalSize::new(
            SCREEN_WIDTH as f64 * dpi_scale,
            SCREEN_HEIGHT as f64 * dpi_scale,
        )
    }

    fn request_hidpi_window_size(window: &Window, dpi_scale: f64) {
        let _ = window.request_inner_size(Self::scaled_logical_size(dpi_scale));
    }

    fn current_dpi_scale(window: &Window) -> f64 {
        window.scale_factor()
    }
}

fn blit_bilinear(src: &[u8], dst: &mut [u32], dst_width: usize, dst_height: usize) {
    let src_w = SCREEN_WIDTH as f32;
    let src_h = SCREEN_HEIGHT as f32;
    let dst_w = dst_width as f32;
    let dst_h = dst_height as f32;

    for y in 0..dst_height {
        let src_yf = ((y as f32 + 0.5) * src_h / dst_h - 0.5).clamp(0.0, src_h - 1.0);
        let y0 = src_yf.floor() as usize;
        let y1 = (y0 + 1).min(SCREEN_HEIGHT - 1);
        let wy = src_yf - y0 as f32;

        for x in 0..dst_width {
            let src_xf = ((x as f32 + 0.5) * src_w / dst_w - 0.5).clamp(0.0, src_w - 1.0);
            let x0 = src_xf.floor() as usize;
            let x1 = (x0 + 1).min(SCREEN_WIDTH - 1);
            let wx = src_xf - x0 as f32;

            let offset00 = y0 * (SCREEN_WIDTH * 4) + x0 * 4;
            let offset10 = y0 * (SCREEN_WIDTH * 4) + x1 * 4;
            let offset01 = y1 * (SCREEN_WIDTH * 4) + x0 * 4;
            let offset11 = y1 * (SCREEN_WIDTH * 4) + x1 * 4;

            let b00 = src[offset00] as f32;
            let g00 = src[offset00 + 1] as f32;
            let r00 = src[offset00 + 2] as f32;
            let b10 = src[offset10] as f32;
            let g10 = src[offset10 + 1] as f32;
            let r10 = src[offset10 + 2] as f32;
            let b01 = src[offset01] as f32;
            let g01 = src[offset01 + 1] as f32;
            let r01 = src[offset01 + 2] as f32;
            let b11 = src[offset11] as f32;
            let g11 = src[offset11 + 1] as f32;
            let r11 = src[offset11 + 2] as f32;

            let r = lerp2d(r00, r10, r01, r11, wx, wy).round() as u32;
            let g = lerp2d(g00, g10, g01, g11, wx, wy).round() as u32;
            let b = lerp2d(b00, b10, b01, b11, wx, wy).round() as u32;

            dst[y * dst_width + x] = (r << 16) | (g << 8) | b;
        }
    }
}

fn lerp2d(v00: f32, v10: f32, v01: f32, v11: f32, wx: f32, wy: f32) -> f32 {
    let top = v00 + (v10 - v00) * wx;
    let bottom = v01 + (v11 - v01) * wx;
    top + (bottom - top) * wy
}

impl ApplicationHandler for SimulationApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes()
            .with_title("WIPI Simulation")
            .with_inner_size(Self::scaled_logical_size(1.0))
            .with_resizable(false);

        let window = Rc::new(event_loop.create_window(window_attrs).unwrap());

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.window = Some(window.clone());
        self.surface = Some(surface);

        Self::request_hidpi_window_size(&window, Self::current_dpi_scale(&window));

        let window_size = window.inner_size();
        self.resize_surface(window_size.width, window_size.height);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.resize_surface(size.width, size.height);
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(window) = &self.window {
                    Self::request_hidpi_window_size(window, Self::current_dpi_scale(window));
                    let size = window.inner_size();
                    self.resize_surface(size.width, size.height);
                }
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

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        kernel::run_due_timers();

        if let Some(window) = &self.window {
            window.request_redraw();
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
