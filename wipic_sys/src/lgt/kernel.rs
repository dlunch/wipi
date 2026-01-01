use core::mem::transmute;

use wipi_boot::lgt::get_external_method;
use wipi_types::lgt::wipic::{ImportModule, WIPICMethod};

pub fn printk(fmt: &str, args: &[*const ()]) {
    let buffer = alloc((fmt.len() + 1) as u32);
    unsafe {
        let buf_ptr = buffer as *mut u8;
        buf_ptr.copy_from_nonoverlapping(fmt.as_ptr(), fmt.len());
        *buf_ptr.add(fmt.len()) = 0; // Null-terminate the string
    }

    let printk: extern "C" fn(*const u8, ...) = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Printk as _,
        ))
    };

    if args.is_empty() {
        printk(buffer as *const u8);
    } else if args.len() == 1 {
        printk(buffer as *const u8, args[0]);
    } else if args.len() == 2 {
        printk(buffer as *const u8, args[0], args[1]);
    } else if args.len() == 3 {
        printk(buffer as *const u8, args[0], args[1], args[2]);
    } else {
        // Handle more arguments as needed
        unimplemented!("printk with more than 3 arguments is not implemented");
    }

    free(buffer);
}

pub fn exit(code: i32) {
    unsafe {
        let exit: extern "C" fn(i32) = transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Exit as _,
        ));

        exit(code);
    }
}

fn alloc(size: u32) -> *const () {
    unsafe {
        let alloc: extern "C" fn(u32) -> *const () = transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Alloc as _,
        ));

        alloc(size)
    }
}

fn free(ptr: *const ()) {
    unsafe {
        let free: extern "C" fn(*const ()) = transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Free as _,
        ));

        free(ptr)
    }
}
