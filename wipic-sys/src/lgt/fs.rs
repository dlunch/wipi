use core::mem::transmute;

use wipi_boot::lgt::get_external_method;
use wipi_types::lgt::wipic::{ImportModule, WIPICMethod};

/// # Safety
/// `name` must point to a valid null-terminated string.
pub unsafe fn open(name: *const u8, mode: i32, access_mode: i32) -> i32 {
    let open: extern "C" fn(*const u8, i32, i32) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::FsOpen as _,
        ))
    };

    open(name, mode, access_mode)
}

pub fn close(fd: i32) -> i32 {
    unsafe {
        let close: extern "C" fn(i32) -> i32 = transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::FsClose as _,
        ));

        close(fd)
    }
}

/// # Safety
/// `buf` must point to at least `len` writable bytes.
pub unsafe fn read(fd: i32, buf: *mut u8, len: i32) -> i32 {
    let read: extern "C" fn(i32, *mut u8, i32) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::FsRead as _,
        ))
    };

    read(fd, buf, len)
}

/// # Safety
/// `buf` must point to at least `len` readable bytes.
pub unsafe fn write(fd: i32, buf: *const u8, len: i32) -> i32 {
    let write: extern "C" fn(i32, *const u8, i32) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::FsWrite as _,
        ))
    };

    write(fd, buf, len)
}
