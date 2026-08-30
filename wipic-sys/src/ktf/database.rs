use core::mem::transmute;

use wipi_boot::ktf::WIPIC_DATABASE_INTERFACE;

/// # Safety
/// `name` must be a valid null-terminated string pointer
pub unsafe fn open_database(name: *const u8, mode: i32, db_type: i32) -> i32 {
    let open_database: extern "C" fn(*const u8, i32, i32) -> i32 =
        unsafe { transmute((*WIPIC_DATABASE_INTERFACE).open_database) };

    open_database(name, mode, db_type)
}

pub fn close_database(db_id: i32) -> i32 {
    let close_database: extern "C" fn(i32) -> i32 =
        unsafe { transmute((*WIPIC_DATABASE_INTERFACE).close_database) };

    close_database(db_id)
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_len` bytes
pub unsafe fn read_record_single(db_id: i32, buf: *mut u8, buf_len: u32) -> i32 {
    let read_record_single: extern "C" fn(i32, *mut u8, u32) -> i32 =
        unsafe { transmute((*WIPIC_DATABASE_INTERFACE).read_record_single) };

    read_record_single(db_id, buf, buf_len)
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_len` bytes
pub unsafe fn write_record_single(db_id: i32, buf: *const u8, buf_len: u32) -> i32 {
    let write_record_single: extern "C" fn(i32, *const u8, u32) -> i32 =
        unsafe { transmute((*WIPIC_DATABASE_INTERFACE).write_record_single) };

    write_record_single(db_id, buf, buf_len)
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_len` bytes
pub unsafe fn select_record(db_id: i32, rec_id: i32, buf: *mut u8, buf_len: u32) -> i32 {
    let select_record: extern "C" fn(i32, i32, *mut u8, u32) -> i32 =
        unsafe { transmute((*WIPIC_DATABASE_INTERFACE).select_record) };

    select_record(db_id, rec_id, buf, buf_len)
}

pub fn delete_record(db_id: i32, rec_id: i32) -> i32 {
    let delete_record: extern "C" fn(i32, i32) -> i32 =
        unsafe { transmute((*WIPIC_DATABASE_INTERFACE).delete_record) };

    delete_record(db_id, rec_id)
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_len` bytes
pub unsafe fn list_record(db_id: i32, buf: *mut u32, buf_len: u32) -> i32 {
    let list_record: extern "C" fn(i32, *mut u32, u32) -> i32 =
        unsafe { transmute((*WIPIC_DATABASE_INTERFACE).list_record) };

    list_record(db_id, buf, buf_len)
}
