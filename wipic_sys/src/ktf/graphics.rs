use core::mem::transmute;

use wipi_boot::ktf::WIPIC_GRAPHICS_INTERFACE;
use wipi_types::wipic::{TargetPtr, WIPICError, WIPICGraphicsContext, WIPICIndirectPtr, WIPICWord};

pub fn get_screen_framebuffer() -> WIPICIndirectPtr {
    let get_screen_framebuffer: extern "C" fn(WIPICWord) -> WIPICIndirectPtr =
        unsafe { transmute((*WIPIC_GRAPHICS_INTERFACE).get_screen_framebuffer) };

    get_screen_framebuffer(0)
}

pub fn flush_lcd(i: i32, framebuffer: WIPICIndirectPtr, x: i32, y: i32, width: u32, height: u32) {
    let flush_lcd: extern "C" fn(i32, TargetPtr, i32, i32, u32, u32) -> () =
        unsafe { transmute((*WIPIC_GRAPHICS_INTERFACE).flush_lcd) };

    flush_lcd(i, framebuffer.0 as _, x, y, width, height);
}

pub unsafe fn init_context(context: *mut WIPICGraphicsContext) {
    let init_context: extern "C" fn(*mut WIPICGraphicsContext) -> () =
        unsafe { transmute((*WIPIC_GRAPHICS_INTERFACE).init_context) };
    init_context(context);
}

pub unsafe fn create_image(
    out_image: *mut WIPICIndirectPtr,
    image_data: WIPICIndirectPtr,
    offset: u32,
    length: u32,
) -> WIPICError {
    let create_image: extern "C" fn(*mut WIPICIndirectPtr, WIPICIndirectPtr, u32, u32) -> i32 =
        unsafe { transmute((*WIPIC_GRAPHICS_INTERFACE).create_image) };
    WIPICError::from_raw(create_image(out_image, image_data, offset, length))
}

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
    ) -> () = unsafe { transmute((*WIPIC_GRAPHICS_INTERFACE).draw_image) };

    draw_image(framebuffer, dx, dy, w, h, image, sx, sy, graphics_context);
}
