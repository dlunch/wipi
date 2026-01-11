extern crate std;

use std::{thread, time::Duration};

use crate::{paint_clet, start_clet};

pub fn simulation_start() {
    unsafe { start_clet() };

    // TODO event loop
    loop {
        unsafe { paint_clet() };
        thread::sleep(Duration::from_millis(16));
    }
}
