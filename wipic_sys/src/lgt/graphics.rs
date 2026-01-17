use core::mem::transmute;

use wipi_boot::lgt::get_external_method;
use wipi_types::{
    lgt::wipic::{ImportModule, WIPICMethod},
    wipic::{TargetPtr, WIPICError, WIPICGraphicsContext, WIPICIndirectPtr},
};

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
pub unsafe fn init_context(context: *mut WIPICGraphicsContext) {
    let init_context: extern "C" fn(*mut WIPICGraphicsContext) -> () = unsafe {
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
    graphics_context: *const WIPICGraphicsContext,
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
        *const WIPICGraphicsContext,
    ) -> () = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::DrawImage as _,
        ))
    };
    draw_image(framebuffer, dx, dy, w, h, image, sx, sy, graphics_context);
}
