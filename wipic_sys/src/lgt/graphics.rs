use core::mem::transmute;

use wipi_boot::lgt::get_external_method;
use wipi_types::{
    lgt::wipic::{ImportModule, WIPICMethod},
    wipic::{TargetPtr, WIPICIndirectPtr},
};

pub fn get_screen_framebuffer() -> WIPICIndirectPtr {
    unsafe {
        let get_screen_framebuffer: extern "C" fn(i32) -> TargetPtr = transmute(
            get_external_method(ImportModule::WIPIC, WIPICMethod::GetScreenFramebuffer as _),
        );

        WIPICIndirectPtr(get_screen_framebuffer(0))
    }
}

pub fn flush_lcd(i: i32, framebuffer: &WIPICIndirectPtr, x: i32, y: i32, width: u32, height: u32) {
    unsafe {
        let flush_lcd: extern "C" fn(i32, TargetPtr, i32, i32, u32, u32) = transmute(
            get_external_method(ImportModule::WIPIC, WIPICMethod::FlushLcd as _),
        );

        flush_lcd(i, framebuffer.0 as _, x, y, width, height)
    }
}
