use wipi_types::wipic::WIPICIndirectPtr;
use wipic_sys::{
    FramebufferInfo,
    graphics::{self, GraphicsContext},
};

use crate::image::Image;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Framebuffer {
    raw: WIPICIndirectPtr,
    info: FramebufferInfo,
    context: GraphicsContext,
}

impl Framebuffer {
    pub fn screen_framebuffer() -> Self {
        let mut context = GraphicsContext::default();
        let raw = graphics::get_screen_framebuffer();

        unsafe { graphics::init_context(&mut context as *mut _) };

        Framebuffer {
            raw,
            info: graphics::framebuffer_info(raw),
            context,
        }
    }

    pub fn width(&self) -> u32 {
        self.info.width
    }

    pub fn height(&self) -> u32 {
        self.info.height
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let offset = y * self.info.bpl + x * (self.info.bpp / 8);
        let pixel = self.color_to_pixel(color);
        unsafe {
            let pixel_ptr = self.info.pixels.add(offset as _);
            match self.info.bpp {
                32 => (pixel_ptr as *mut u32).write_unaligned(pixel),
                16 => (pixel_ptr as *mut u16).write_unaligned(pixel as u16),
                _ => panic!("Unsupported bpp: {}", self.info.bpp),
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_image(
        &mut self,
        dx: i32,
        dy: i32,
        w: u32,
        h: u32,
        image: &Image,
        sx: i32,
        sy: i32,
    ) {
        unsafe {
            graphics::draw_image(
                self.raw,
                dx,
                dy,
                w,
                h,
                image.raw(),
                sx,
                sy,
                &self.context as *const _,
            );
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        self.set_foreground(color);
        unsafe {
            graphics::draw_rect(self.raw, x, y, width, height, &self.context as *const _);
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        self.set_foreground(color);
        unsafe {
            graphics::fill_rect(self.raw, x, y, width, height, &self.context as *const _);
        }
    }

    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, color: Color) {
        self.set_foreground(color);
        cfg_if::cfg_if! {
            if #[cfg(any(feature = "ktf", feature = "lgt"))] {
                let text = crate::euc_kr::encode(text);
            } else {
                let text = text.as_bytes();
            }
        }
        unsafe {
            graphics::draw_string(
                self.raw,
                x,
                y,
                text.as_ptr(),
                text.len() as i32,
                &self.context as *const _,
            );
        }
    }

    pub fn request_repaint(&self) {
        graphics::repaint(0, 0, 0, self.info.width as i32, self.info.height as i32);
    }

    fn set_foreground(&mut self, color: Color) {
        let pixel = self.color_to_pixel(color);
        graphics::set_foreground(&mut self.context, pixel);
    }

    /// Pixel value in the framebuffer's native format, as the platform draw calls expect.
    fn color_to_pixel(&self, color: Color) -> u32 {
        if self.info.bpp == 16 {
            ((color.r as u32 >> 3) << 11) | ((color.g as u32 >> 2) << 5) | (color.b as u32 >> 3)
        } else {
            ((color.a as u32) << 24)
                | ((color.r as u32) << 16)
                | ((color.g as u32) << 8)
                | (color.b as u32)
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        graphics::flush_lcd(0, self.raw, 0, 0, self.info.width, self.info.height);
    }
}
