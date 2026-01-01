use wipi_types::wipic::WIPICIndirectPtr;

pub fn get_screen_framebuffer() -> WIPICIndirectPtr {
    wipic_simulation::graphics::get_screen_framebuffer()
}

pub fn flush_lcd(i: i32, framebuffer: &WIPICIndirectPtr, x: i32, y: i32, width: u32, height: u32) {
    wipic_simulation::graphics::flush_lcd(i, framebuffer, x, y, width, height);
}
