use core::mem::transmute;

use wipi_boot::lgt::get_external_method;
use wipi_types::lgt::wipic::{ImportModule, WIPICMethod};

/// # Safety
/// `name` must be a valid null-terminated string pointer
pub unsafe fn open_database(name: *const u8, mode: i32, db_type: i32) -> i32 {
    let open_database: extern "C" fn(*const u8, i32, i32) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::OpenDatabase as _,
        ))
    };

    open_database(name, mode, db_type)
}

pub fn close_database(db_id: i32) -> i32 {
    unsafe {
        let close_database: extern "C" fn(i32) -> i32 = transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::CloseDatabase as _,
        ));

        close_database(db_id)
    }
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_len` bytes
pub unsafe fn read_record_single(db_id: i32, buf: *mut u8, buf_len: u32) -> i32 {
    let read_record_single: extern "C" fn(i32, *mut u8, u32) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::ReadRecordSingle as _,
        ))
    };

    read_record_single(db_id, buf, buf_len)
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_len` bytes
pub unsafe fn write_record_single(db_id: i32, buf: *const u8, buf_len: u32) -> i32 {
    let write_record_single: extern "C" fn(i32, *const u8, u32) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::WriteRecordSingle as _,
        ))
    };

    write_record_single(db_id, buf, buf_len)
}
