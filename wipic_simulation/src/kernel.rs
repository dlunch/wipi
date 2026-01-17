use core::ffi::{CStr, c_char};
use std::alloc::Layout;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{fs, print, process};

use once_cell::sync::Lazy;

static RESOURCES: Lazy<Mutex<ResourceManager>> = Lazy::new(|| Mutex::new(ResourceManager::new()));
static ALLOCATIONS: Lazy<Mutex<HashMap<usize, Layout>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static RESOURCE_PATH: Lazy<Mutex<PathBuf>> = Lazy::new(|| Mutex::new(PathBuf::from("resources")));

pub fn set_resource_path(path: PathBuf) {
    *RESOURCE_PATH.lock().unwrap() = path;
}

struct ResourceManager {
    next_id: i32,
    resources: HashMap<i32, Vec<u8>>,
}

impl ResourceManager {
    fn new() -> Self {
        Self {
            next_id: 0,
            resources: HashMap::new(),
        }
    }

    fn load_resource(&mut self, path: &str) -> Option<(i32, usize)> {
        let base_path = RESOURCE_PATH.lock().unwrap();
        let full_path = base_path.join(path);
        let data = fs::read(&full_path).ok()?;
        let size = data.len();
        let id = self.next_id;
        self.next_id += 1;
        self.resources.insert(id, data);
        Some((id, size))
    }

    fn get_resource(&self, id: i32) -> Option<&[u8]> {
        self.resources.get(&id).map(|v| v.as_slice())
    }
}

pub fn printk(fmt: &str, _args: &[*const ()]) {
    print!("{fmt}");
}

pub fn exit(code: i32) {
    process::exit(code);
}

pub fn alloc(size: u32) -> *mut u8 {
    let layout = Layout::from_size_align(size as usize, 4).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };

    ALLOCATIONS.lock().unwrap().insert(ptr as usize, layout);

    ptr
}

/// # Safety
/// `ptr` must be a valid pointer previously returned by `alloc`
pub unsafe fn free(ptr: *mut u8) {
    if let Some(layout) = ALLOCATIONS.lock().unwrap().remove(&(ptr as usize)) {
        unsafe { std::alloc::dealloc(ptr, layout) };
    }
}

/// # Safety
/// `name` must be a valid null-terminated C string, `out_size` must be a valid pointer
pub unsafe fn get_resource_id(name: *const c_char, out_size: *mut usize) -> i32 {
    let name_str = unsafe { CStr::from_ptr(name) };
    let path = name_str.to_str().unwrap_or("");

    let mut manager = RESOURCES.lock().unwrap();
    match manager.load_resource(path) {
        Some((id, size)) => {
            unsafe { *out_size = size };
            id
        }
        None => -12,
    }
}

/// # Safety
/// `buf` must be a valid pointer with at least `buf_size` bytes
pub unsafe fn get_resource(id: i32, buf: *mut u8, buf_size: usize) -> i32 {
    let manager = RESOURCES.lock().unwrap();
    match manager.get_resource(id) {
        Some(data) => {
            if data.len() > buf_size {
                return -18;
            }
            unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), buf, data.len()) };
            0
        }
        None => -12,
    }
}
