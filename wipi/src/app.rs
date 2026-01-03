use alloc::boxed::Box;

static mut APP: Option<Box<dyn App>> = None;

pub trait App {
    fn on_paint(&mut self);
    fn on_pause(&mut self);
    fn on_resume(&mut self);
    fn on_event(&mut self);
}

fn register_app(app: Box<dyn App>) {
    unsafe {
        APP = Some(app);
    }
}

unsafe extern "Rust" {
    fn __wipi_main() -> Box<dyn App>;
}

#[unsafe(export_name = "startClet")]
extern "C" fn start_clet() {
    let app = unsafe { __wipi_main() };
    register_app(app);
}

#[unsafe(export_name = "destroyClet")]
extern "C" fn destroy_clet() {}

#[unsafe(export_name = "paintClet")]
#[allow(static_mut_refs)]
extern "C" fn paint_clet() {
    unsafe {
        if let Some(app) = APP.as_mut() {
            app.on_paint();
        }
    }
}

#[unsafe(export_name = "pauseClet")]
extern "C" fn pause_clet() {}

#[unsafe(export_name = "resumeClet")]
extern "C" fn resume_clet() {}

#[unsafe(export_name = "handleCletEvent")]
extern "C" fn handle_clet_event() {}
