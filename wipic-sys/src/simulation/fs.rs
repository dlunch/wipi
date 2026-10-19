/// # Safety
/// `name` must point to a valid null-terminated string.
pub unsafe fn open(name: *const u8, mode: i32, access_mode: i32) -> i32 {
    unsafe { wipic_simulation::fs::open(name, mode, access_mode) }
}

pub fn close(fd: i32) -> i32 {
    wipic_simulation::fs::close(fd)
}

/// # Safety
/// `buf` must point to at least `len` writable bytes.
pub unsafe fn read(fd: i32, buf: *mut u8, len: i32) -> i32 {
    unsafe { wipic_simulation::fs::read(fd, buf, len) }
}

/// # Safety
/// `buf` must point to at least `len` readable bytes.
pub unsafe fn write(fd: i32, buf: *const u8, len: i32) -> i32 {
    unsafe { wipic_simulation::fs::write(fd, buf, len) }
}
