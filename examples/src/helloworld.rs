#![no_std]
#![no_main]

use wipi::print;

#[unsafe(export_name = "startClet")]
extern "C" fn start_clet() {
    print("Hello, world!");
}

#[unsafe(export_name = "destroyClet")]
extern "C" fn destroy_clet() {
    print("Destroying Clet!");
}

#[unsafe(export_name = "paintClet")]
extern "C" fn paint_clet() {
    print("Painting Clet!");
}

#[unsafe(export_name = "pauseClet")]
extern "C" fn pause_clet() {
    print("Pausing Clet!");
}

#[unsafe(export_name = "resumeClet")]
extern "C" fn resume_clet() {
    print("Resuming Clet!");
}
