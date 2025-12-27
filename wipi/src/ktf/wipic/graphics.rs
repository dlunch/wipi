use core::mem::transmute;

use wipi_types::wipic::{WIPICIndirectPtr, WIPICWord};

use crate::ktf::globals::WIPIC_GRAPHICS_INTERFACE;

pub fn get_screen_framebuffer() -> WIPICIndirectPtr {
    let get_screen_framebuffer: extern "C" fn(WIPICWord) -> WIPICIndirectPtr =
        unsafe { transmute((*WIPIC_GRAPHICS_INTERFACE).get_screen_framebuffer) };

    get_screen_framebuffer(0)
}
