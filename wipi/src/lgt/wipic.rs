use core::mem::transmute;

use crate::lgt::external::get_external_method;

pub fn println(s: &str) {
    let buffer = alloc((s.len() + 1) as u32);
    unsafe {
        let buf_ptr = buffer as *mut u8;
        buf_ptr.copy_from_nonoverlapping(s.as_ptr(), s.len());
        *buf_ptr.add(s.len()) = 0; // Null-terminate the string
    }

    unsafe {
        let printk: extern "C" fn(*const u8) = transmute(get_external_method(0x1fb, 0x64));
        printk(buffer as *const u8);
    }

    free(buffer);
}

pub fn exit(code: i32) {
    unsafe {
        let exit: extern "C" fn(i32) = transmute(get_external_method(0x1fb, 0x6b));

        exit(code);
    }
}

fn alloc(size: u32) -> *const () {
    unsafe {
        let alloc: extern "C" fn(u32) -> *const () = transmute(get_external_method(0x1fb, 0x75));

        alloc(size)
    }
}

fn free(ptr: *const ()) {
    unsafe {
        let free: extern "C" fn(*const ()) = transmute(get_external_method(0x1fb, 0x77));

        free(ptr)
    }
}
