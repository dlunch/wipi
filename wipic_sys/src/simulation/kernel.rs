use core::ffi::c_char;

use wipi_types::wipic::{WIPICError, WIPICIndirectPtr};

pub fn printk(fmt: &str, args: &[*const ()]) {
    wipic_simulation::kernel::printk(fmt, args);
}

pub fn exit(code: i32) {
    wipic_simulation::kernel::exit(code);
}

pub fn alloc(size: u32) -> WIPICIndirectPtr {
    wipic_simulation::kernel::alloc(size)
}

pub fn free(ptr: WIPICIndirectPtr) {
    wipic_simulation::kernel::free(ptr);
}

pub fn get_resource_id(name: *const c_char, out_size: *mut usize) -> i32 {
    wipic_simulation::kernel::get_resource_id(name, out_size)
}

pub fn get_resource(id: i32, buf: WIPICIndirectPtr, buf_size: usize) -> WIPICError {
    wipic_simulation::kernel::get_resource(id, buf, buf_size)
}
