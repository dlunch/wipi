#![no_std]
#![no_main]

use core::mem::transmute;

use wipi::kernel::{exit, printk};

#[unsafe(export_name = "startClet")]
extern "C" fn start_clet() {
    unsafe {
        printk(
            "Hello, %s! %d",
            &[c"world".as_ptr() as _, transmute(2024_usize)],
        )
    };
    exit(0);
}

#[unsafe(export_name = "destroyClet")]
extern "C" fn destroy_clet() {}

#[unsafe(export_name = "paintClet")]
extern "C" fn paint_clet() {}

#[unsafe(export_name = "pauseClet")]
extern "C" fn pause_clet() {}

#[unsafe(export_name = "resumeClet")]
extern "C" fn resume_clet() {}

#[unsafe(export_name = "handleCletEvent")]
extern "C" fn handle_clet_event() {}
