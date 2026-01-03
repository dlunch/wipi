#![allow(static_mut_refs)]

use alloc::boxed::Box;

use crate::app::App;

unsafe extern "Rust" {
    fn __wipi_main() -> Box<dyn App>;
}

static mut APP: Option<Box<dyn App>> = None;

#[unsafe(export_name = "startClet")]
extern "C" fn start_clet() {
    let app = unsafe { __wipi_main() };
    unsafe {
        APP = Some(app);
    }
}

#[unsafe(export_name = "destroyClet")]
extern "C" fn destroy_clet() {}

#[unsafe(export_name = "paintClet")]
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
extern "C" fn handle_clet_event(r#type: i32, param1: i32, param2: i32) {
    unsafe {
        if let Some(app) = APP.as_mut() {
            app.on_raw_event(r#type, param1, param2);
        }
    }
}
