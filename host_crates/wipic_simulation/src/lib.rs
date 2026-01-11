pub mod graphics;
pub mod kernel;

use std::{thread, time::Duration};

pub fn simulation_start(start_clet: unsafe extern "C" fn(), paint_clet: unsafe extern "C" fn()) {
    unsafe { start_clet() };

    // TODO event loop
    loop {
        unsafe { paint_clet() };
        thread::sleep(Duration::from_millis(16));
    }
}
