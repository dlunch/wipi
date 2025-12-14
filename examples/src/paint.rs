#![no_std]
#![no_main]

use wipi::println;

#[unsafe(export_name = "startClet")]
extern "C" fn start_clet() {
    println("Triangle Clet started");
}

#[unsafe(export_name = "destroyClet")]
extern "C" fn destroy_clet() {}

#[unsafe(export_name = "paintClet")]
extern "C" fn paint_clet() {
    println("paintClet called");
}

#[unsafe(export_name = "pauseClet")]
extern "C" fn pause_clet() {}

#[unsafe(export_name = "resumeClet")]
extern "C" fn resume_clet() {}

#[unsafe(export_name = "handleCletEvent")]
extern "C" fn handle_clet_event() {}
