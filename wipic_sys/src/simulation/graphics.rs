use wipi_types::wipic::{WIPICError, WIPICGraphicsContext, WIPICIndirectPtr};

pub fn get_screen_framebuffer() -> WIPICIndirectPtr {
    wipic_simulation::graphics::get_screen_framebuffer()
}

pub fn flush_lcd(i: i32, framebuffer: WIPICIndirectPtr, x: i32, y: i32, width: u32, height: u32) {
    wipic_simulation::graphics::flush_lcd(i, framebuffer, x, y, width, height);
}

pub fn init_context(context: *mut WIPICGraphicsContext) {
    wipic_simulation::graphics::init_context(context);
}

pub fn create_image(
    out_image: *mut WIPICIndirectPtr,
    image_data: WIPICIndirectPtr,
    offset: u32,
    length: u32,
) -> WIPICError {
    wipic_simulation::graphics::create_image(out_image, image_data, offset, length)
}

#[allow(clippy::too_many_arguments)]
pub fn draw_image(
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
    wipic_simulation::graphics::draw_image(
        framebuffer,
        dx,
        dy,
        w,
        h,
        image,
        sx,
        sy,
        graphics_context,
    );
}
