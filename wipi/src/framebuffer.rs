use wipi_types::wipic::{WIPICFramebuffer, WIPICGraphicsContext, WIPICIndirectPtr};
use wipic_sys::deref_indirect_ptr;

use crate::image::Image;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Framebuffer {
    raw: WIPICIndirectPtr,
    context: WIPICGraphicsContext,
}

impl Framebuffer {
    pub fn screen_framebuffer() -> Self {
        let mut context = WIPICGraphicsContext::default();
        let raw = wipic_sys::graphics::get_screen_framebuffer();

        unsafe { wipic_sys::graphics::init_context(&mut context as *mut _) };

        Framebuffer { raw, context }
    }

    pub fn width(&self) -> u32 {
        self.read_fb().width as _
    }

    pub fn height(&self) -> u32 {
        self.read_fb().height as _
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let fb = self.read_fb();
        let buffer_ptr = Self::buffer_ptr(fb);

        let bpl: u32 = fb.bpl as _;
        let bpp: u32 = fb.bpp as _;

        let offset = y * bpl + x * (bpp / 8);
        unsafe {
            let pixel_ptr = buffer_ptr.add(offset as _);
            match fb.bpp {
                32 => {
                    *pixel_ptr.add(0) = color.b;
                    *pixel_ptr.add(1) = color.g;
                    *pixel_ptr.add(2) = color.r;
                    *pixel_ptr.add(3) = color.a;
                }
                16 => {
                    let r = (color.r as u16 >> 3) & 0x1F;
                    let g = (color.g as u16 >> 2) & 0x3F;
                    let b = (color.b as u16 >> 3) & 0x1F;
                    let pixel_value = (r << 11) | (g << 5) | b;
                    *(pixel_ptr as *mut u16) = pixel_value;
                }
                _ => {
                    panic!("Unsupported bpp: {}", fb.bpp);
                }
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
            wipic_sys::graphics::draw_image(
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
        self.context.fgpxl = Self::color_to_pixel(color) as _;
        unsafe {
            wipic_sys::graphics::draw_rect(
                self.raw,
                x,
                y,
                width,
                height,
                &self.context as *const _,
            );
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        self.context.fgpxl = Self::color_to_pixel(color) as _;
        unsafe {
            wipic_sys::graphics::fill_rect(
                self.raw,
                x,
                y,
                width,
                height,
                &self.context as *const _,
            );
        }
    }

    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, color: Color) {
        self.context.fgpxl = Self::color_to_pixel(color) as _;
        unsafe {
            wipic_sys::graphics::draw_string(
                self.raw,
                x,
                y,
                text.as_ptr(),
                text.len() as i32,
                &self.context as *const _,
            );
        }
    }

    fn color_to_pixel(color: Color) -> u32 {
        ((color.a as u32) << 24)
            | ((color.r as u32) << 16)
            | ((color.g as u32) << 8)
            | (color.b as u32)
    }

    fn read_fb(&self) -> &WIPICFramebuffer {
        unsafe { &*(deref_indirect_ptr(self.raw) as *const _) }
    }

    fn buffer_ptr(fb: &WIPICFramebuffer) -> *mut u8 {
        deref_indirect_ptr(fb.buf)
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        let fb = self.read_fb();
        wipic_sys::graphics::flush_lcd(0, self.raw, 0, 0, fb.width as _, fb.height as _);
    }
}
