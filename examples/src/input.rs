#![cfg_attr(not(test), no_main)]
#![no_std]
extern crate alloc;

use wipi::{app::App, event::KeyCode, println, wipi_main};

#[derive(Default)]
pub struct InputApp;

impl InputApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for InputApp {
    fn on_keydown(&mut self, key_code: KeyCode) {
        println!("Key down: {:?}", key_code);
    }

    fn on_keyup(&mut self, key_code: KeyCode) {
        println!("Key up: {:?}", key_code);
    }
}

#[wipi_main]
pub fn main() -> InputApp {
    InputApp::new()
}
