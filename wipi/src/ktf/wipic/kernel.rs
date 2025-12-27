use core::{ffi::c_char, mem::transmute};

use wipi_types::wipic::WIPICIndirectPtr;

use crate::ktf::{globals::WIPIC_KNLINTERFACE, wipic::deref_indirect_ptr};

pub fn println(s: &str) {
    let printk: extern "C" fn(*const c_char, ...) -> () =
        unsafe { transmute((*WIPIC_KNLINTERFACE).printk) };

    let null_terminated_buf = alloc((s.len() + 1) as _);
    unsafe {
        let buf_ptr = deref_indirect_ptr(null_terminated_buf);
        buf_ptr.copy_from_nonoverlapping(s.as_ptr(), s.len());
        *buf_ptr.add(s.len()) = 0; // Null-terminate the string
    }

    printk(deref_indirect_ptr(null_terminated_buf) as _);

    free(null_terminated_buf);
}

pub fn exit(code: i32) {
    let exit: extern "C" fn(i32) = unsafe { transmute((*WIPIC_KNLINTERFACE).exit) };
    exit(code);
}

pub fn alloc(size: u32) -> WIPICIndirectPtr {
    let alloc: extern "C" fn(u32) -> *mut u8 = unsafe { transmute((*WIPIC_KNLINTERFACE).alloc) };

    WIPICIndirectPtr(alloc(size) as _)
}

pub fn free(ptr: WIPICIndirectPtr) {
    let free: extern "C" fn(*mut u8) = unsafe { transmute((*WIPIC_KNLINTERFACE).free) };

    free(ptr.0 as _)
}
