use core::mem::transmute;

use wipi_boot::lgt::get_external_method;
use wipi_types::{
    lgt::wipic::{
        ImportModule, LgtFramebuffer, LgtGraphicsContext, LgtGraphicsView, LgtImage, WIPICMethod,
    },
    wipic::{TargetPtr, WIPICError, WIPICFramebuffer, WIPICImage, WIPICIndirectPtr},
};

use crate::FramebufferInfo;

pub type GraphicsContext = LgtGraphicsContext;

pub fn set_foreground(context: &mut GraphicsContext, pixel: u32) {
    context.foreground = pixel;
}

pub fn framebuffer_info(framebuffer: WIPICIndirectPtr) -> FramebufferInfo {
    // An LGT framebuffer handle points at a view into a backing pixel buffer.
    unsafe {
        let framebuffer = &*(framebuffer.0 as *const LgtFramebuffer);
        let view = &*(framebuffer.ptr_graphics as *const LgtGraphicsView);
        let backing = &*(view.ptr_backing as *const WIPICFramebuffer);
        let offset = view.y as u32 * backing.bpl + view.x as u32 * (backing.bpp / 8);

        FramebufferInfo {
            width: view.width,
            height: view.height,
            bpl: backing.bpl,
            bpp: backing.bpp,
            pixels: (backing.buf.0 as *mut u8).add(offset as usize),
        }
    }
}

pub fn image_size(image: WIPICIndirectPtr) -> (u32, u32) {
    unsafe {
        let image = &*(image.0 as *const LgtImage);
        let image = &*(image.ptr_image as *const WIPICImage);

        (image.img.width, image.img.height)
    }
}

pub fn get_screen_framebuffer() -> WIPICIndirectPtr {
    unsafe {
        let get_screen_framebuffer: extern "C" fn(i32) -> TargetPtr = transmute(
            get_external_method(ImportModule::WIPIC, WIPICMethod::GetScreenFramebuffer as _),
        );

        WIPICIndirectPtr(get_screen_framebuffer(0))
    }
}

pub fn flush_lcd(i: i32, framebuffer: WIPICIndirectPtr, x: i32, y: i32, width: u32, height: u32) {
    unsafe {
        let flush_lcd: extern "C" fn(i32, TargetPtr, i32, i32, u32, u32) = transmute(
            get_external_method(ImportModule::WIPIC, WIPICMethod::FlushLcd as _),
        );

        flush_lcd(i, framebuffer.0 as _, x, y, width, height)
    }
}

/// # Safety
/// The caller must ensure that the pointer passed to this function is valid.
pub unsafe fn init_context(context: *mut GraphicsContext) {
    let init_context: extern "C" fn(*mut GraphicsContext) -> () = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::InitContext as _,
        ))
    };
    init_context(context);
}

/// # Safety
/// The caller must ensure that the pointers passed to this function are valid.
pub unsafe fn create_image(
    out_image: *mut WIPICIndirectPtr,
    image_data: WIPICIndirectPtr,
    offset: u32,
    length: u32,
) -> WIPICError {
    let create_image: extern "C" fn(*mut WIPICIndirectPtr, WIPICIndirectPtr, u32, u32) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::CreateImage as _,
        ))
    };
    WIPICError::from_raw(create_image(out_image, image_data, offset, length))
}

/// # Safety
/// The caller must ensure that the pointers passed to this function are valid.
#[allow(clippy::too_many_arguments)]
pub unsafe fn draw_image(
    framebuffer: WIPICIndirectPtr,
    dx: i32,
    dy: i32,
    w: u32,
    h: u32,
    image: WIPICIndirectPtr,
    sx: i32,
    sy: i32,
    graphics_context: *const GraphicsContext,
) {
    let draw_image: extern "C" fn(
        WIPICIndirectPtr,
        i32,
        i32,
        u32,
        u32,
        WIPICIndirectPtr,
        i32,
        i32,
        *const GraphicsContext,
    ) -> () = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::DrawImage as _,
        ))
    };
    draw_image(framebuffer, dx, dy, w, h, image, sx, sy, graphics_context);
}

/// # Safety
/// The caller must ensure that the pointers passed to this function are valid.
pub unsafe fn draw_rect(
    framebuffer: WIPICIndirectPtr,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    graphics_context: *const GraphicsContext,
) {
    let draw_rect: extern "C" fn(WIPICIndirectPtr, i32, i32, i32, i32, *const GraphicsContext) = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::DrawRect as _,
        ))
    };
    draw_rect(framebuffer, x, y, width, height, graphics_context);
}

/// # Safety
/// The caller must ensure that the pointers passed to this function are valid.
pub unsafe fn fill_rect(
    framebuffer: WIPICIndirectPtr,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    graphics_context: *const GraphicsContext,
) {
    let fill_rect: extern "C" fn(WIPICIndirectPtr, i32, i32, i32, i32, *const GraphicsContext) = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::FillRect as _,
        ))
    };
    fill_rect(framebuffer, x, y, width, height, graphics_context);
}

/// # Safety
/// The caller must ensure that the pointers passed to this function are valid.
pub unsafe fn draw_string(
    framebuffer: WIPICIndirectPtr,
    x: i32,
    y: i32,
    string: *const u8,
    length: i32,
    graphics_context: *const GraphicsContext,
) {
    let draw_string: extern "C" fn(
        WIPICIndirectPtr,
        i32,
        i32,
        *const u8,
        i32,
        *const GraphicsContext,
    ) = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::DrawString as _,
        ))
    };
    draw_string(framebuffer, x, y, string, length, graphics_context);
}

pub fn repaint(lcd: i32, x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        let repaint: extern "C" fn(i32, i32, i32, i32, i32) = transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Repaint as _,
        ));

        repaint(lcd, x, y, width, height);
    }
}
