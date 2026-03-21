use std::fs;
use std::io::Cursor;
use std::sync::Mutex;

use ab_glyph::{Font, FontArc, FontVec, GlyphId, PxScale, ScaleFont, point};
use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use once_cell::sync::Lazy;
use wipi_types::wipic::{
    WIPICError, WIPICFramebuffer, WIPICGraphicsContext, WIPICImage, WIPICIndirectPtr,
};

use crate::kernel::alloc;

pub const SCREEN_WIDTH: usize = 240;
pub const SCREEN_HEIGHT: usize = 320;
const SCREEN_BPP: usize = 32;
const SCREEN_BPL: usize = SCREEN_WIDTH * (SCREEN_BPP / 8);
const TAB_SPACES: i32 = 4;
const SYSTEM_FONT_SIZE: f32 = 13.0;

pub static SCREEN_FRAMEBUFFER: Lazy<Mutex<ScreenFramebuffer>> =
    Lazy::new(|| Mutex::new(ScreenFramebuffer::new()));
static SYSTEM_FONT: Lazy<Option<FontArc>> = Lazy::new(load_system_font);

fn load_system_font() -> Option<FontArc> {
    let source = SystemSource::new();
    let families = [FamilyName::SansSerif];
    let handle = source
        .select_best_match(&families, &Properties::new())
        .ok()?;

    let (font_data, font_index) = match handle {
        Handle::Path { path, font_index } => (fs::read(path).ok()?, font_index),
        Handle::Memory { bytes, font_index } => (bytes.as_ref().to_vec(), font_index),
    };

    let font = FontVec::try_from_vec_and_index(font_data, font_index).ok()?;
    Some(FontArc::new(font))
}

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
    ctx.clip = [0, 0, SCREEN_WIDTH as _, SCREEN_HEIGHT as _];
    ctx.fgpxl = (-1) as _;
    ctx.bgpxl = 0x0;
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
    framebuffer: *mut WIPICFramebuffer,
    x: i32,
    y: i32,
    string: *const u8,
    length: i32,
    graphics_context: *const WIPICGraphicsContext,
) {
    if framebuffer.is_null() || graphics_context.is_null() || string.is_null() || length <= 0 {
        return;
    }

    let Some(font) = SYSTEM_FONT.as_ref() else {
        return;
    };

    let fb = unsafe { &*framebuffer };
    let gctx = unsafe { &*graphics_context };
    let dst_buf = fb.buf.0 as *mut u8;
    if dst_buf.is_null() {
        return;
    }

    let bytes = unsafe { std::slice::from_raw_parts(string, length as usize) };
    let text = String::from_utf8_lossy(bytes);
    draw_system_font_string(dst_buf, fb, gctx, x, y, &text, font);
}

pub fn request_repaint() {
    // simulation already repaints every frame
}

#[derive(Clone, Copy)]
struct ClipRect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl ClipRect {
    fn from_context(context: &WIPICGraphicsContext) -> Self {
        Self {
            left: context.clip[0] as i32,
            top: context.clip[1] as i32,
            right: context.clip[2] as i32,
            bottom: context.clip[3] as i32,
        }
    }

    fn contains(self, x: i32, y: i32) -> bool {
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }
}

fn draw_system_font_string(
    dst_buf: *mut u8,
    fb: &WIPICFramebuffer,
    gctx: &WIPICGraphicsContext,
    x: i32,
    y: i32,
    text: &str,
    font: &FontArc,
) {
    let clip = ClipRect::from_context(gctx);
    let color = gctx.fgpxl as u32;
    let scale = PxScale::from(SYSTEM_FONT_SIZE);
    let scaled = font.as_scaled(scale);

    let mut line_height = scaled.height().ceil() as i32;
    if line_height <= 0 {
        line_height = SYSTEM_FONT_SIZE.ceil() as i32;
    }

    let mut space_advance = scaled.h_advance(font.glyph_id(' ')).ceil() as i32;
    if space_advance <= 0 {
        space_advance = 1;
    }

    let origin_x = x + gctx.offset[0] as i32;
    let mut cursor_x = origin_x;
    let mut cursor_y = y + gctx.offset[1] as i32;
    let ascent = scaled.ascent().ceil() as i32;

    for ch in text.chars() {
        match ch {
            '\r' => continue,
            '\n' => {
                cursor_x = origin_x;
                cursor_y += line_height;
                continue;
            }
            '\t' => {
                cursor_x += space_advance * TAB_SPACES;
                continue;
            }
            _ => {}
        }

        let mut glyph_id = font.glyph_id(ch);
        if glyph_id.0 == 0 {
            glyph_id = font.glyph_id('\u{FFFD}');
        }
        if glyph_id.0 == 0 {
            cursor_x += space_advance;
            continue;
        }

        let baseline_y = cursor_y + ascent;
        draw_grayscale_glyph(
            dst_buf, fb, font, glyph_id, scale, cursor_x, baseline_y, color, clip,
        );

        let mut advance = scaled.h_advance(glyph_id).ceil() as i32;
        if advance <= 0 {
            advance = 1;
        }
        cursor_x += advance;
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_grayscale_glyph(
    dst_buf: *mut u8,
    fb: &WIPICFramebuffer,
    font: &FontArc,
    glyph_id: GlyphId,
    scale: PxScale,
    x: i32,
    baseline_y: i32,
    color: u32,
    clip: ClipRect,
) {
    let glyph = glyph_id.with_scale_and_position(scale, point(x as f32, baseline_y as f32));
    if let Some(outlined) = font.outline_glyph(glyph) {
        let bounds = outlined.px_bounds();
        let min_x = bounds.min.x.floor() as i32;
        let min_y = bounds.min.y.floor() as i32;
        outlined.draw(|glyph_x, glyph_y, coverage| {
            blend_pixel(
                dst_buf,
                fb,
                min_x + glyph_x as i32,
                min_y + glyph_y as i32,
                color,
                coverage,
                clip,
            );
        });
    }
}

fn blend_pixel(
    dst_buf: *mut u8,
    fb: &WIPICFramebuffer,
    x: i32,
    y: i32,
    color: u32,
    coverage: f32,
    clip: ClipRect,
) {
    if coverage <= 0.0 {
        return;
    }
    if x < 0 || y < 0 || x >= fb.width as i32 || y >= fb.height as i32 || !clip.contains(x, y) {
        return;
    }

    let bpp = fb.bpp;
    if !bpp.is_multiple_of(8) {
        return;
    }

    let src_alpha = ((color >> 24) & 0xFF) as f32 / 255.0;
    let alpha = (src_alpha * coverage).clamp(0.0, 1.0);
    if alpha <= 0.0 {
        return;
    }

    let bytes_per_pixel = bpp / 8;
    let offset = y as usize * fb.bpl + x as usize * bytes_per_pixel;

    let src_b = (color & 0xFF) as u8;
    let src_g = ((color >> 8) & 0xFF) as u8;
    let src_r = ((color >> 16) & 0xFF) as u8;

    unsafe {
        match bpp {
            32 => {
                let dst = std::ptr::read_unaligned(dst_buf.add(offset) as *const u32);
                let dst_b = (dst & 0xFF) as u8;
                let dst_g = ((dst >> 8) & 0xFF) as u8;
                let dst_r = ((dst >> 16) & 0xFF) as u8;
                let dst_a = ((dst >> 24) & 0xFF) as u8;

                let out_b = blend_channel(src_b, dst_b, alpha);
                let out_g = blend_channel(src_g, dst_g, alpha);
                let out_r = blend_channel(src_r, dst_r, alpha);
                let out_a = blend_channel(255, dst_a, alpha);

                let out = ((out_a as u32) << 24)
                    | ((out_r as u32) << 16)
                    | ((out_g as u32) << 8)
                    | (out_b as u32);
                std::ptr::write_unaligned(dst_buf.add(offset) as *mut u32, out);
            }
            24 => {
                let dst_b = *dst_buf.add(offset);
                let dst_g = *dst_buf.add(offset + 1);
                let dst_r = *dst_buf.add(offset + 2);

                *dst_buf.add(offset) = blend_channel(src_b, dst_b, alpha);
                *dst_buf.add(offset + 1) = blend_channel(src_g, dst_g, alpha);
                *dst_buf.add(offset + 2) = blend_channel(src_r, dst_r, alpha);
            }
            16 if alpha >= 0.5 => {
                let rgb565 = (((src_r as u16) >> 3) << 11)
                    | (((src_g as u16) >> 2) << 5)
                    | ((src_b as u16) >> 3);
                std::ptr::write_unaligned(dst_buf.add(offset) as *mut u16, rgb565);
            }
            8 if alpha >= 0.5 => {
                *dst_buf.add(offset) =
                    ((src_r as u16 * 30 + src_g as u16 * 59 + src_b as u16 * 11) / 100) as u8;
            }
            _ => {}
        }
    }
}

fn blend_channel(src: u8, dst: u8, alpha: f32) -> u8 {
    (src as f32 * alpha + dst as f32 * (1.0 - alpha)).round() as u8
}
