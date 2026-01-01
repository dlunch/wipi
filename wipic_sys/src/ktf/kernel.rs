use core::{ffi::c_char, mem::transmute};

use wipi_boot::ktf::WIPIC_KNLINTERFACE;
use wipi_types::wipic::WIPICIndirectPtr;

use crate::ktf::deref_indirect_ptr;

// TODO macro for variable arguments
pub fn printk(fmt: &str, args: &[*const ()]) {
    let printk: extern "C" fn(*const c_char, ...) -> () =
        unsafe { transmute((*WIPIC_KNLINTERFACE).printk) };

    let null_terminated_buf = alloc((fmt.len() + 1) as _);
    unsafe {
        let buf_ptr = deref_indirect_ptr(null_terminated_buf);
        buf_ptr.copy_from_nonoverlapping(fmt.as_ptr(), fmt.len());
        *buf_ptr.add(fmt.len()) = 0; // Null-terminate the string
    }

    // TODO is it right?..
    if args.is_empty() {
        printk(deref_indirect_ptr(null_terminated_buf) as _);
    } else if args.len() == 1 {
        printk(deref_indirect_ptr(null_terminated_buf) as _, args[0]);
    } else if args.len() == 2 {
        printk(
            deref_indirect_ptr(null_terminated_buf) as _,
            args[0],
            args[1],
        );
    } else if args.len() == 3 {
        printk(
            deref_indirect_ptr(null_terminated_buf) as _,
            args[0],
            args[1],
            args[2],
        );
    } else {
        // Handle more arguments as needed
        unimplemented!("printk with more than 3 arguments is not implemented");
    }

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
