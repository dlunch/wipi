#![no_std]
#![cfg_attr(not(test), no_main)]
extern crate alloc;

use wipi::{app::App, framebuffer::Framebuffer, image::Image, wipi_main};

pub struct ImageApp {
    image: Image,
}

impl ImageApp {
    pub fn new(image_path: &str) -> Self {
        let image = Image::new(image_path).unwrap();
        Self { image }
    }
}

impl App for ImageApp {
    fn on_paint(&mut self) {
        let mut fb = Framebuffer::screen_framebuffer();

        fb.draw_image(
            0,
            0,
            self.image.width(),
            self.image.height(),
            &self.image,
            0,
            0,
        );
    }
}

#[wipi_main]
pub fn main() -> ImageApp {
    ImageApp::new("image.png")
}
