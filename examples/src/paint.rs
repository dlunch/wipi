#![no_std]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

use wipi::{
    app::App,
    framebuffer::{Color, Framebuffer},
    wipi_main,
};

#[derive(Default)]
pub struct PaintApp;

impl PaintApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for PaintApp {
    fn on_paint(&mut self) {
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

        fb.draw_rect(
            20,
            20,
            50,
            30,
            Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        );

        fb.fill_rect(
            100,
            50,
            60,
            40,
            Color {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            },
        );

        fb.draw_text(
            20,
            100,
            "Hello WIPI!",
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        );
    }
}

#[wipi_main]
pub fn main() -> PaintApp {
    PaintApp::new()
}
