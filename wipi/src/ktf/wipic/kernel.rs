use core::{ffi::c_char, mem::transmute};

use crate::ktf::{globals::WIPIC_KNLINTERFACE, wipic::KtfWipiCMemory};

pub fn println(s: &str) {
    let printk: extern "C" fn(*const c_char, ...) -> () =
        unsafe { transmute((*WIPIC_KNLINTERFACE).printk) };

    let null_terminated_buf = alloc((s.len() + 1) as _);
    unsafe {
        let buf_ptr = null_terminated_buf.as_ptr();
        buf_ptr.copy_from_nonoverlapping(s.as_ptr(), s.len());
        *buf_ptr.add(s.len()) = 0; // Null-terminate the string
    }

    printk(null_terminated_buf.as_ptr() as *const c_char);

    free(null_terminated_buf);
}

pub fn exit(code: i32) {
    let exit: extern "C" fn(i32) = unsafe { transmute((*WIPIC_KNLINTERFACE).exit) };
    exit(code);
}

pub fn alloc(size: u32) -> KtfWipiCMemory {
    let alloc: extern "C" fn(u32) -> *mut u8 = unsafe { transmute((*WIPIC_KNLINTERFACE).alloc) };

    KtfWipiCMemory(alloc(size))
}

pub fn free(ptr: KtfWipiCMemory) {
    let free: extern "C" fn(*mut u8) = unsafe { transmute((*WIPIC_KNLINTERFACE).free) };

    free(ptr.0)
}
