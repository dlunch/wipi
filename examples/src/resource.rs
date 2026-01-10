#![no_std]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

use wipi::{app::App, kernel::exit, println, wipi_main};

#[derive(Default)]
pub struct ResourceApp;

impl ResourceApp {
    pub fn new() -> Self {
        let resource = wipi::resource::Resource::new("test.txt").unwrap();
        let data = resource.read();
        println!("{:?}", data);

        exit(0);

        Self {}
    }
}

impl App for ResourceApp {}

#[wipi_main]
pub fn main() -> ResourceApp {
    ResourceApp::new()
}
