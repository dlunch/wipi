use std::io::Cursor;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use wipi_types::wipic::{
    WIPICError, WIPICFramebuffer, WIPICGraphicsContext, WIPICImage, WIPICIndirectPtr,
};

use crate::kernel::alloc;

pub const SCREEN_WIDTH: usize = 240;
pub const SCREEN_HEIGHT: usize = 320;
const SCREEN_BPP: usize = 32;
const SCREEN_BPL: usize = SCREEN_WIDTH * (SCREEN_BPP / 8);

pub static SCREEN_FRAMEBUFFER: Lazy<Mutex<ScreenFramebuffer>> =
    Lazy::new(|| Mutex::new(ScreenFramebuffer::new()));

pub struct ScreenFramebuffer {
    framebuffer_ptr: *mut WIPICFramebuffer,
    buffer_ptr: *mut u8,
}

unsafe impl Send for ScreenFramebuffer {}

impl ScreenFramebuffer {
    fn new() -> Self {
        let buffer_size = SCREEN_BPL * SCREEN_HEIGHT;
        let buffer_ptr = alloc(buffer_size as u32);

        unsafe {
            std::ptr::write_bytes(buffer_ptr, 0, buffer_size);
        }

        let framebuffer_ptr =
            alloc(std::mem::size_of::<WIPICFramebuffer>() as u32) as *mut WIPICFramebuffer;
        let fb = WIPICFramebuffer {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            bpl: SCREEN_BPL,
            bpp: SCREEN_BPP,
            buf: WIPICIndirectPtr(buffer_ptr as _),
        };
        unsafe {
            std::ptr::write(framebuffer_ptr, fb);
        }

        Self {
            framebuffer_ptr,
            buffer_ptr,
        }
    }

    pub fn get_framebuffer_ptr(&self) -> *mut WIPICFramebuffer {
        self.framebuffer_ptr
    }

    pub fn buffer(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.buffer_ptr, SCREEN_BPL * SCREEN_HEIGHT) }
    }
}

pub fn get_screen_framebuffer() -> *mut WIPICFramebuffer {
    SCREEN_FRAMEBUFFER.lock().unwrap().get_framebuffer_ptr()
}

pub fn flush_lcd(
    _i: i32,
    _framebuffer: *mut WIPICFramebuffer,
    _x: i32,
    _y: i32,
    _width: u32,
    _height: u32,
) {
}

/// # Safety
/// `context` must be a valid pointer
pub unsafe fn init_context(context: *mut WIPICGraphicsContext) {
    let ctx = unsafe { &mut *context };
    ctx.mask = 0;
    ctx.clip = [0, 0, SCREEN_WIDTH, SCREEN_HEIGHT];
    ctx.fgpxl = 0xFFFFFFFF;
    ctx.bgpxl = 0x00000000;
    ctx.transpxl = 0;
    ctx.alpha = 255;
    ctx.offset = [0, 0];
    ctx.pixel_op_func_ptr = 0;
    ctx.param1 = 0;
    ctx.reserved = 0;
    ctx.font = 0;
    ctx.style = 0;
}

pub fn create_image(data: &[u8]) -> Result<WIPICImage, WIPICError> {
    let decoder = png::Decoder::new(Cursor::new(data));
    let mut reader = decoder.read_info().map_err(|_| WIPICError::Invalid)?;

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buf)
        .map_err(|_| WIPICError::Invalid)?;

    let width = info.width as usize;
    let height = info.height as usize;
    let bpp: usize = match info.color_type {
        png::ColorType::Rgba => 32,
        png::ColorType::Rgb => 24,
        png::ColorType::GrayscaleAlpha => 16,
        png::ColorType::Grayscale => 8,
        _ => 32,
    };

    let bpl = width * (bpp / 8);
    let buffer_size = bpl * height;
    let img_buf_ptr = alloc(buffer_size as u32);

    let src_bpp: usize = match info.color_type {
        png::ColorType::Rgba => 4,
        png::ColorType::Rgb => 3,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Grayscale => 1,
        _ => 4,
    };

    unsafe {
        let dst = img_buf_ptr;
        for y in 0..height {
            for x in 0..width {
                let src_offset = y * width * src_bpp + x * src_bpp;
                let dst_offset = y * bpl + x * (bpp / 8);

                match info.color_type {
                    png::ColorType::Rgba => {
                        *dst.add(dst_offset) = buf[src_offset + 2];
                        *dst.add(dst_offset + 1) = buf[src_offset + 1];
                        *dst.add(dst_offset + 2) = buf[src_offset];
                        *dst.add(dst_offset + 3) = buf[src_offset + 3];
                    }
                    png::ColorType::Rgb => {
                        *dst.add(dst_offset) = buf[src_offset + 2];
                        *dst.add(dst_offset + 1) = buf[src_offset + 1];
                        *dst.add(dst_offset + 2) = buf[src_offset];
                    }
                    _ => {
                        for i in 0..src_bpp {
                            *dst.add(dst_offset + i) = buf[src_offset + i];
                        }
                    }
                }
            }
        }
    }

    let img_fb = WIPICFramebuffer {
        width,
        height,
        bpl,
        bpp,
        buf: WIPICIndirectPtr(img_buf_ptr as _),
    };

    let mask_fb = WIPICFramebuffer {
        width: 0,
        height: 0,
        bpl: 0,
        bpp: 0,
        buf: WIPICIndirectPtr(0),
    };

    let image = WIPICImage {
        img: img_fb,
        mask: mask_fb,
        loop_count: 0,
        delay: 0,
        animated: 0,
        buf: WIPICIndirectPtr(0),
        offset: 0,
        current: 0,
        len: data.len(),
    };

    Ok(image)
}

/// # Safety
/// All pointers must be valid
pub unsafe fn draw_rect(
    framebuffer: *mut WIPICFramebuffer,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    graphics_context: *const WIPICGraphicsContext,
) {
    let fb = unsafe { &*framebuffer };
    let gctx = unsafe { &*graphics_context };
    let dst_buf = fb.buf.0 as *mut u8;
    let color = gctx.fgpxl as u32;

    let x = x.max(0) as usize;
    let y = y.max(0) as usize;
    let width = width.max(0) as usize;
    let height = height.max(0) as usize;

    for i in 0..width {
        let px = x + i;
        if px >= fb.width {
            continue;
        }
        for py in [y, y + height.saturating_sub(1)] {
            if py >= fb.height {
                continue;
            }
            let offset = py * fb.bpl + px * (fb.bpp / 8);
            unsafe {
                std::ptr::write(dst_buf.add(offset) as *mut u32, color);
            }
        }
    }

    for j in 0..height {
        let py = y + j;
        if py >= fb.height {
            continue;
        }
        for px in [x, x + width.saturating_sub(1)] {
            if px >= fb.width {
                continue;
            }
            let offset = py * fb.bpl + px * (fb.bpp / 8);
            unsafe {
                std::ptr::write(dst_buf.add(offset) as *mut u32, color);
            }
        }
    }
}

/// # Safety
/// All pointers must be valid
pub unsafe fn fill_rect(
    framebuffer: *mut WIPICFramebuffer,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    graphics_context: *const WIPICGraphicsContext,
) {
    let fb = unsafe { &*framebuffer };
    let gctx = unsafe { &*graphics_context };
    let dst_buf = fb.buf.0 as *mut u8;
    let color = gctx.fgpxl as u32;

    let x = x.max(0) as usize;
    let y = y.max(0) as usize;
    let width = width.max(0) as usize;
    let height = height.max(0) as usize;

    for j in 0..height {
        let py = y + j;
        if py >= fb.height {
            continue;
        }
        for i in 0..width {
            let px = x + i;
            if px >= fb.width {
                continue;
            }
            let offset = py * fb.bpl + px * (fb.bpp / 8);
            unsafe {
                std::ptr::write(dst_buf.add(offset) as *mut u32, color);
            }
        }
    }
}

/// # Safety
/// All pointers must be valid
#[allow(clippy::too_many_arguments)]
pub unsafe fn draw_image(
    framebuffer: *mut WIPICFramebuffer,
    dx: i32,
    dy: i32,
    w: u32,
    h: u32,
    image: *const WIPICImage,
    sx: i32,
    sy: i32,
    _graphics_context: *const WIPICGraphicsContext,
) {
    let fb = unsafe { &*framebuffer };
    let img = unsafe { &*image };

    let dst_buf = fb.buf.0 as *mut u8;
    let src_buf = img.img.buf.0 as *const u8;

    let src_bpp = img.img.bpp / 8;
    let dst_bpp = fb.bpp / 8;

    for row in 0..h as usize {
        let src_y = sy as usize + row;
        let dst_y = dy as usize + row;

        if src_y >= img.img.height || dst_y >= fb.height {
            continue;
        }

        for col in 0..w as usize {
            let src_x = sx as usize + col;
            let dst_x = dx as usize + col;

            if src_x >= img.img.width || dst_x >= fb.width {
                continue;
            }

            let src_offset = src_y * img.img.bpl + src_x * src_bpp;
            let dst_offset = dst_y * fb.bpl + dst_x * dst_bpp;

            unsafe {
                let bytes_to_copy = std::cmp::min(src_bpp, dst_bpp) as usize;
                for i in 0..bytes_to_copy {
                    *dst_buf.add(dst_offset + i) = *src_buf.add(src_offset + i);
                }
                if dst_bpp == 4 && src_bpp == 3 {
                    *dst_buf.add(dst_offset + 3) = 255;
                }
            }
        }
    }
}

/// # Safety
/// All pointers must be valid
pub unsafe fn draw_string(
    _framebuffer: *mut WIPICFramebuffer,
    _x: i32,
    _y: i32,
    _string: *const u8,
    _length: i32,
    _graphics_context: *const WIPICGraphicsContext,
) {
    // TODO: implement text rendering
}

pub fn request_repaint() {
    // simulation already repaints every frame
}
