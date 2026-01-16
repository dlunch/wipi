use wipi_types::wipic::{WIPICError, WIPICGraphicsContext, WIPICIndirectPtr};

pub fn get_screen_framebuffer() -> WIPICIndirectPtr {
    // TODO
    WIPICIndirectPtr(0)
}

pub fn flush_lcd(
    _i: i32,
    _framebuffer: WIPICIndirectPtr,
    _x: i32,
    _y: i32,
    _width: u32,
    _height: u32,
) {
    // TODO
}

pub fn init_context(_context: *mut WIPICGraphicsContext) {}

pub fn create_image(
    _out_image: *mut WIPICIndirectPtr,
    _image_data: WIPICIndirectPtr,
    _offset: u32,
    _length: u32,
) -> WIPICError {
    todo!()
}

#[allow(clippy::too_many_arguments)]
pub fn draw_image(
    _framebuffer: WIPICIndirectPtr,
    _dx: i32,
    _dy: i32,
    _w: u32,
    _h: u32,
    _image: WIPICIndirectPtr,
    _sx: i32,
    _sy: i32,
    _graphics_context: *const WIPICGraphicsContext,
) {
    todo!()
}
