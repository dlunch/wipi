#![no_std]
#![no_main]
extern crate alloc;

use wipi::{app::App, kernel::exit, println};
use wipi_macros::wipi_main;

#[derive(Default)]
pub struct HelloWorldApp;

impl HelloWorldApp {
    pub fn new() -> Self {
        println!("Hello, world! {}", 2024);
        exit(0);

        Self {}
    }
}

impl App for HelloWorldApp {}

#[wipi_main]
pub fn wipi_main() -> HelloWorldApp {
    HelloWorldApp::new()
}
