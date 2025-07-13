#![no_std]
#![no_main]

use wipi::{exit, println};

#[unsafe(export_name = "startClet")]
extern "C" fn start_clet() {
    println("Hello, world!");
    exit(0);
}

#[unsafe(export_name = "destroyClet")]
extern "C" fn destroy_clet() {
    println("Destroying Clet!");
}

#[unsafe(export_name = "paintClet")]
extern "C" fn paint_clet() {
    println("Painting Clet!");
}

#[unsafe(export_name = "pauseClet")]
extern "C" fn pause_clet() {
    println("Pausing Clet!");
}

#[unsafe(export_name = "resumeClet")]
extern "C" fn resume_clet() {
    println("Resuming Clet!");
}
