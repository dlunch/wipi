/// # Safety
/// `name` must be a valid null-terminated string pointer
pub unsafe fn open_database(name: *const u8, mode: i32, db_type: i32) -> i32 {
    unsafe { wipic_simulation::database::open_database(name, mode, db_type) }
}

pub fn close_database(db_id: i32) -> i32 {
    wipic_simulation::database::close_database(db_id)
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_len` bytes
pub unsafe fn read_record_single(db_id: i32, buf: *mut u8, buf_len: u32) -> i32 {
    unsafe { wipic_simulation::database::read_record_single(db_id, buf, buf_len) }
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_len` bytes
pub unsafe fn write_record_single(db_id: i32, buf: *const u8, buf_len: u32) -> i32 {
    unsafe { wipic_simulation::database::write_record_single(db_id, buf, buf_len) }
}
