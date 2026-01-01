use wipi_types::wipic::WIPICIndirectPtr;

pub fn get_screen_framebuffer() -> WIPICIndirectPtr {
    // TODO
    WIPICIndirectPtr(0)
}

pub fn flush_lcd(
    _i: i32,
    _framebuffer: &WIPICIndirectPtr,
    _x: i32,
    _y: i32,
    _width: u32,
    _height: u32,
) {
    // TODO
}
