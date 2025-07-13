use core::{ffi::c_char, mem::transmute};

use super::globals::WIPIC_KNLINTERFACE;

#[repr(transparent)]
pub struct KtfWipiMemory(*mut u8);

impl KtfWipiMemory {
    pub fn as_ptr(&self) -> *mut u8 {
        unsafe { self.0.add(12) }
    }
}

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

pub fn alloc(size: u32) -> KtfWipiMemory {
    let alloc: extern "C" fn(u32) -> *mut u8 = unsafe { transmute((*WIPIC_KNLINTERFACE).alloc) };

    KtfWipiMemory(alloc(size))
}

pub fn free(ptr: KtfWipiMemory) {
    let free: extern "C" fn(*mut u8) = unsafe { transmute((*WIPIC_KNLINTERFACE).free) };

    free(ptr.0)
}
