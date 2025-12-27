use core::mem::transmute;

use wipi_types::wipic::{TargetPtr, WIPICIndirectPtr, WIPICWord};

use crate::ktf::globals::WIPIC_GRAPHICS_INTERFACE;

pub fn get_screen_framebuffer() -> WIPICIndirectPtr {
    let get_screen_framebuffer: extern "C" fn(WIPICWord) -> WIPICIndirectPtr =
        unsafe { transmute((*WIPIC_GRAPHICS_INTERFACE).get_screen_framebuffer) };

    get_screen_framebuffer(0)
}

pub fn flush_lcd(i: i32, framebuffer: WIPICIndirectPtr, x: i32, y: i32, width: i32, height: i32) {
    let flush_lcd: extern "C" fn(i32, TargetPtr, i32, i32, i32, i32) -> () =
        unsafe { transmute((*WIPIC_GRAPHICS_INTERFACE).flush_lcd) };

    flush_lcd(i, framebuffer.0 as _, x, y, width, height);
}
