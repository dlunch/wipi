use core::ffi::c_char;
use std::{print, process};

use wipi_types::wipic::{WIPICError, WIPICIndirectPtr};

pub fn printk(fmt: &str, _args: &[*const ()]) {
    print!("{fmt}");
}

pub fn exit(code: i32) {
    process::exit(code);
}

pub fn alloc(_size: u32) -> WIPICIndirectPtr {
    todo!()
}

pub fn free(_ptr: WIPICIndirectPtr) {
    todo!()
}

pub fn get_resource_id(_name: *const c_char, _out_size: *mut usize) -> i32 {
    todo!()
}

pub fn get_resource(_id: i32, _buf: WIPICIndirectPtr, _buf_size: usize) -> WIPICError {
    todo!()
}
