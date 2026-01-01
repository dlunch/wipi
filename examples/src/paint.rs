#![no_std]
#![no_main]

use wipi::{Color, Framebuffer, println};

#[unsafe(export_name = "startClet")]
extern "C" fn start_clet() {
    println!("paint started");
}

#[unsafe(export_name = "destroyClet")]
extern "C" fn destroy_clet() {}

#[unsafe(export_name = "paintClet")]
extern "C" fn paint_clet() {
    let mut fb = Framebuffer::screen_framebuffer();

    fb.set_pixel(
        10,
        10,
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        },
    );
}

#[unsafe(export_name = "pauseClet")]
extern "C" fn pause_clet() {}

#[unsafe(export_name = "resumeClet")]
extern "C" fn resume_clet() {}

#[unsafe(export_name = "handleCletEvent")]
extern "C" fn handle_clet_event() {}
