use core::{mem, ptr, slice};

use wipi_types::wipic::{
    WIPICError, WIPICFramebuffer, WIPICGraphicsContext, WIPICImage, WIPICIndirectPtr,
};
use wipic_simulation::kernel::alloc;

use crate::deref_indirect_ptr;

pub fn get_screen_framebuffer() -> WIPICIndirectPtr {
    let ptr = wipic_simulation::graphics::get_screen_framebuffer();
    unsafe { crate::to_indirect_ptr(ptr as *mut u8) }
}

pub fn flush_lcd(i: i32, framebuffer: WIPICIndirectPtr, x: i32, y: i32, width: u32, height: u32) {
    wipic_simulation::graphics::flush_lcd(
        i,
        deref_indirect_ptr(framebuffer) as *mut WIPICFramebuffer,
        x,
        y,
        width,
        height,
    );
}

/// # Safety
/// `context` must be a valid pointer
pub unsafe fn init_context(context: *mut WIPICGraphicsContext) {
    unsafe { wipic_simulation::graphics::init_context(context) };
}

/// # Safety
/// `out_image` must be a valid pointer
pub unsafe fn create_image(
    out_image: *mut WIPICIndirectPtr,
    image_data: WIPICIndirectPtr,
    offset: u32,
    length: u32,
) -> WIPICError {
    let data = unsafe { slice::from_raw_parts(deref_indirect_ptr(image_data), length as usize) };

    let result = wipic_simulation::graphics::create_image(&data[offset as usize..]);

    match result {
        Ok(image) => {
            let ptr = alloc(mem::size_of::<WIPICImage>() as u32);
            unsafe {
                ptr::write(ptr as *mut WIPICImage, image);
                ptr::write(out_image, crate::to_indirect_ptr(ptr));
            }

            WIPICError::ImageDone
        }
        Err(e) => e,
    }
}

/// # Safety
/// `graphics_context` must be a valid pointer
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
    unsafe {
        wipic_simulation::graphics::draw_image(
            deref_indirect_ptr(framebuffer) as *mut WIPICFramebuffer,
            dx,
            dy,
            w,
            h,
            deref_indirect_ptr(image) as *const WIPICImage,
            sx,
            sy,
            graphics_context,
        )
    };
}

/// # Safety
/// `graphics_context` must be a valid pointer
pub unsafe fn draw_rect(
    framebuffer: WIPICIndirectPtr,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    graphics_context: *const WIPICGraphicsContext,
) {
    unsafe {
        wipic_simulation::graphics::draw_rect(
            deref_indirect_ptr(framebuffer) as *mut WIPICFramebuffer,
            x,
            y,
            width,
            height,
            graphics_context,
        )
    };
}

/// # Safety
/// `graphics_context` must be a valid pointer
pub unsafe fn fill_rect(
    framebuffer: WIPICIndirectPtr,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    graphics_context: *const WIPICGraphicsContext,
) {
    unsafe {
        wipic_simulation::graphics::fill_rect(
            deref_indirect_ptr(framebuffer) as *mut WIPICFramebuffer,
            x,
            y,
            width,
            height,
            graphics_context,
        )
    };
}

/// # Safety
/// `graphics_context` must be a valid pointer
pub unsafe fn draw_string(
    framebuffer: WIPICIndirectPtr,
    x: i32,
    y: i32,
    string: *const u8,
    length: i32,
    graphics_context: *const WIPICGraphicsContext,
) {
    unsafe {
        wipic_simulation::graphics::draw_string(
            deref_indirect_ptr(framebuffer) as *mut WIPICFramebuffer,
            x,
            y,
            string,
            length,
            graphics_context,
        )
    };
}
