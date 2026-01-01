use wipi_types::wipic::{WIPICFramebuffer, WIPICIndirectPtr};
use wipic_sys::deref_indirect_ptr;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Framebuffer {
    raw: WIPICIndirectPtr,
}

impl Framebuffer {
    pub fn screen_framebuffer() -> Self {
        let raw = wipic_sys::graphics::get_screen_framebuffer();
        Framebuffer { raw }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let fb = self.read_fb();
        let buffer_ptr = Self::buffer_ptr(fb);

        let offset = y * fb.bpl as usize + x * (fb.bpp as usize / 8);
        unsafe {
            let pixel_ptr = buffer_ptr.add(offset);
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

    fn read_fb(&self) -> &WIPICFramebuffer {
        unsafe { &*(deref_indirect_ptr(&self.raw) as *const WIPICFramebuffer) }
    }

    fn buffer_ptr(fb: &WIPICFramebuffer) -> *mut u8 {
        deref_indirect_ptr(&fb.buf)
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        let fb = self.read_fb();
        wipic_sys::graphics::flush_lcd(0, &self.raw, 0, 0, fb.width, fb.height);
    }
}
