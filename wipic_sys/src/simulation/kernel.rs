use core::ffi::c_char;

use wipi_types::wipic::{WIPICError, WIPICIndirectPtr};

use crate::deref_indirect_ptr;

pub fn printk(fmt: &str, args: &[*const ()]) {
    wipic_simulation::kernel::printk(fmt, args);
}

pub fn exit(code: i32) {
    wipic_simulation::kernel::exit(code);
}

pub fn alloc(size: u32) -> WIPICIndirectPtr {
    let ptr = wipic_simulation::kernel::alloc(size);
    unsafe { crate::to_indirect_ptr(ptr) }
}

pub fn free(ptr: WIPICIndirectPtr) {
    unsafe { wipic_simulation::kernel::free(deref_indirect_ptr(ptr)) };
}

/// # Safety
/// `name` must be a valid null-terminated C string, `out_size` must be a valid pointer
pub unsafe fn get_resource_id(name: *const c_char, out_size: *mut usize) -> i32 {
    unsafe { wipic_simulation::kernel::get_resource_id(name, out_size) }
}

pub fn get_resource(id: i32, buf: WIPICIndirectPtr, buf_size: usize) -> WIPICError {
    let result =
        unsafe { wipic_simulation::kernel::get_resource(id, deref_indirect_ptr(buf), buf_size) };
    WIPICError::from_raw(result)
}
