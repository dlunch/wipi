use alloc::boxed::Box;
use core::cell::RefCell;

use crate::app::App;

unsafe extern "Rust" {
    fn __wipi_main() -> Box<dyn App>;
}

struct SyncRefCell<T>(RefCell<T>);
// SAFETY: Lifecycle callbacks are only called from the main thread.
unsafe impl<T> Sync for SyncRefCell<T> {}

static APP: SyncRefCell<Option<Box<dyn App>>> = SyncRefCell(RefCell::new(None));

#[unsafe(export_name = "startClet")]
extern "C" fn start_clet() {
    let app = unsafe { __wipi_main() };
    *APP.0.borrow_mut() = Some(app);
}

#[unsafe(export_name = "destroyClet")]
extern "C" fn destroy_clet() {}

#[unsafe(export_name = "paintClet")]
extern "C" fn paint_clet() {
    if let Some(app) = APP.0.borrow_mut().as_mut() {
        app.on_paint();
    }
}

#[unsafe(export_name = "pauseClet")]
extern "C" fn pause_clet() {}

#[unsafe(export_name = "resumeClet")]
extern "C" fn resume_clet() {}

#[unsafe(export_name = "handleCletEvent")]
extern "C" fn handle_clet_event(r#type: i32, param1: i32, param2: i32) {
    if let Some(app) = APP.0.borrow_mut().as_mut() {
        app.on_raw_event(r#type, param1, param2);
    }
}
