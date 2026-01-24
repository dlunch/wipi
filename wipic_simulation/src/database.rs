use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use wipi_types::wipic::WIPICError;

static DATABASES: Lazy<Mutex<HashMap<i32, Database>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static NEXT_DB_ID: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(1));

struct Database {
    _name: String,
    data: Vec<u8>,
}

/// # Safety
/// `name` must be a valid null-terminated string pointer
pub unsafe fn open_database(name: *const u8, _mode: i32, _db_type: i32) -> i32 {
    let name_str = unsafe { CStr::from_ptr(name as *const i8) }
        .to_string_lossy()
        .into_owned();

    let mut next_id = NEXT_DB_ID.lock().unwrap();
    let db_id = *next_id;
    *next_id += 1;

    let db = Database {
        _name: name_str,
        data: Vec::new(),
    };

    DATABASES.lock().unwrap().insert(db_id, db);

    db_id
}

pub fn close_database(db_id: i32) -> i32 {
    DATABASES.lock().unwrap().remove(&db_id);
    WIPICError::Success as i32
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_len` bytes
pub unsafe fn read_record_single(db_id: i32, buf: *mut u8, buf_len: u32) -> i32 {
    let databases = DATABASES.lock().unwrap();
    if let Some(db) = databases.get(&db_id) {
        if db.data.is_empty() {
            return WIPICError::BadRecordId as i32;
        }
        let len = db.data.len().min(buf_len as usize);
        unsafe {
            std::ptr::copy_nonoverlapping(db.data.as_ptr(), buf, len);
        }
        len as i32
    } else {
        WIPICError::InvalidHandle as i32
    }
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_len` bytes
pub unsafe fn write_record_single(db_id: i32, buf: *const u8, buf_len: u32) -> i32 {
    let mut databases = DATABASES.lock().unwrap();
    if let Some(db) = databases.get_mut(&db_id) {
        db.data = unsafe { std::slice::from_raw_parts(buf, buf_len as usize) }.to_vec();
        buf_len as i32
    } else {
        WIPICError::InvalidHandle as i32
    }
}
