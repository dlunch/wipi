#![cfg_attr(target_os = "none", no_std)]
#![cfg(not(target_os = "none"))]

pub mod graphics;
pub mod kernel;

use std::{thread, time::Duration};

pub fn simulation_start(
    start_clet: unsafe extern "C" fn(),
    paint_clet: unsafe extern "C" fn(),
    _handle_input: unsafe extern "C" fn(i32, i32, i32),
) {
    unsafe { start_clet() };

    // TODO event loop
    loop {
        unsafe { paint_clet() };
        thread::sleep(Duration::from_millis(16));
    }
}
