use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::Mutex;

use once_cell::sync::Lazy;

static FILES: Lazy<Mutex<HashMap<String, Vec<u8>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static HANDLES: Lazy<Mutex<HashMap<i32, FileHandle>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static NEXT_FD: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(1));

struct FileHandle {
    name: String,
    position: usize,
    mode: i32,
}

/// # Safety
/// `name` must point to a valid null-terminated string.
pub unsafe fn open(name: *const u8, mode: i32, _access_mode: i32) -> i32 {
    let name = unsafe { CStr::from_ptr(name.cast()) }
        .to_string_lossy()
        .into_owned();
    let mut files = FILES.lock().unwrap();

    match mode {
        1 if !files.contains_key(&name) => return -12,
        1 | 2 | 8 => {
            files.entry(name.clone()).or_default();
        }
        4 => {
            files.insert(name.clone(), Vec::new());
        }
        _ => return -1,
    }
    drop(files);

    let mut next_fd = NEXT_FD.lock().unwrap();
    let fd = *next_fd;
    *next_fd += 1;
    HANDLES.lock().unwrap().insert(
        fd,
        FileHandle {
            name,
            position: 0,
            mode,
        },
    );
    fd
}

pub fn close(fd: i32) -> i32 {
    if HANDLES.lock().unwrap().remove(&fd).is_some() {
        0
    } else {
        -2
    }
}

/// # Safety
/// `buf` must point to at least `len` writable bytes.
pub unsafe fn read(fd: i32, buf: *mut u8, len: i32) -> i32 {
    if len < 0 {
        return -1;
    }
    if len == 0 {
        return 0;
    }

    let mut handles = HANDLES.lock().unwrap();
    let Some(handle) = handles.get_mut(&fd) else {
        return -2;
    };
    if matches!(handle.mode, 2 | 4) {
        return -24;
    }

    let files = FILES.lock().unwrap();
    let data = &files[&handle.name];
    let count = (len as usize).min(data.len().saturating_sub(handle.position));
    if count == 0 {
        return -23;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr().add(handle.position), buf, count);
    }
    handle.position += count;
    count as i32
}

/// # Safety
/// `buf` must point to at least `len` readable bytes.
pub unsafe fn write(fd: i32, buf: *const u8, len: i32) -> i32 {
    if len < 0 {
        return -1;
    }

    let mut handles = HANDLES.lock().unwrap();
    let Some(handle) = handles.get_mut(&fd) else {
        return -2;
    };
    if handle.mode == 1 {
        return -24;
    }

    let len = len as usize;
    let Some(end) = handle.position.checked_add(len) else {
        return -1;
    };
    let mut files = FILES.lock().unwrap();
    let data = files.get_mut(&handle.name).unwrap();
    data.resize(end.max(data.len()), 0);
    unsafe {
        std::ptr::copy_nonoverlapping(buf, data.as_mut_ptr().add(handle.position), len);
    }
    handle.position = end;
    len as i32
}

#[cfg(test)]
mod tests {
    use super::{close, open, read, write};

    #[test]
    fn file_contents_survive_close_and_reopen() {
        let name = c"file_contents_survive_close_and_reopen";
        let fd = unsafe { open(name.as_ptr().cast(), 4, 1) };
        assert!(fd > 0);
        assert_eq!(unsafe { write(fd, b"data".as_ptr(), 4) }, 4);
        assert_eq!(close(fd), 0);

        let fd = unsafe { open(name.as_ptr().cast(), 1, 1) };
        let mut data = [0; 4];
        assert_eq!(unsafe { read(fd, data.as_mut_ptr(), 4) }, 4);
        assert_eq!(&data, b"data");
        assert_eq!(close(fd), 0);
    }
}
