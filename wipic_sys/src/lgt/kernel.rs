use core::{ffi::c_char, mem::transmute};

use wipi_boot::lgt::get_external_method;
use wipi_types::{
    lgt::wipic::{ImportModule, WIPICMethod},
    wipic::{WIPICError, WIPICIndirectPtr},
};

use crate::deref_indirect_ptr;

pub fn printk(fmt: &str, args: &[*const ()]) {
    let buffer = alloc((fmt.len() + 1) as u32);
    let buf_ptr = deref_indirect_ptr(buffer);
    unsafe {
        buf_ptr.copy_from_nonoverlapping(fmt.as_ptr(), fmt.len());
        *buf_ptr.add(fmt.len()) = 0; // Null-terminate the string
    }

    let printk: extern "C" fn(*const c_char, ...) = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Printk as _,
        ))
    };

    if args.is_empty() {
        printk(buf_ptr as *const _);
    } else if args.len() == 1 {
        printk(buf_ptr as *const _, args[0]);
    } else if args.len() == 2 {
        printk(buf_ptr as *const _, args[0], args[1]);
    } else if args.len() == 3 {
        printk(buf_ptr as *const _, args[0], args[1], args[2]);
    } else {
        // Handle more arguments as needed
        unimplemented!("printk with more than 3 arguments is not implemented");
    }

    free(buffer);
}

pub fn exit(code: i32) {
    let exit: extern "C" fn(i32) = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Exit as _,
        ))
    };

    exit(code);
}

pub fn alloc(size: u32) -> WIPICIndirectPtr {
    let alloc: extern "C" fn(u32) -> *const () = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Alloc as _,
        ))
    };

    WIPICIndirectPtr(alloc(size))
}

pub fn free(ptr: WIPICIndirectPtr) {
    let free: extern "C" fn(*const ()) = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::Free as _,
        ))
    };

    free(ptr.0 as _)
}

/// # Safety
/// The caller must ensure that `name` is a valid null-terminated C string.
pub unsafe fn get_resource_id(name: *const c_char, out_size: *mut usize) -> i32 {
    let get_resource_id: extern "C" fn(*const c_char, *mut usize) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::GetResourceId as _,
        ))
    };

    get_resource_id(name, out_size)
}

pub fn get_resource(id: i32, buf: WIPICIndirectPtr, buf_size: usize) -> WIPICError {
    let get_resource: extern "C" fn(i32, *mut u8, usize) -> i32 = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::GetResource as _,
        ))
    };

    WIPICError::from_raw(get_resource(id, buf.0 as _, buf_size))
}

pub fn def_timer(timer: *mut u8, callback: extern "C" fn(*mut u8, *mut u8)) {
    let def_timer: extern "C" fn(*mut u8, extern "C" fn(*mut u8, *mut u8)) = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::DefTimer as _,
        ))
    };
    def_timer(timer, callback);
}

pub fn set_timer(timer: *mut u8, timeout_low: u32, timeout_high: u32, param: *mut u8) {
    let set_timer: extern "C" fn(*mut u8, u32, u32, *mut u8) = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::SetTimer as _,
        ))
    };
    set_timer(timer, timeout_low, timeout_high, param);
}

pub fn unset_timer(timer: *mut u8) {
    let unset_timer: extern "C" fn(*mut u8) = unsafe {
        transmute(get_external_method(
            ImportModule::WIPIC,
            WIPICMethod::UnsetTimer as _,
        ))
    };
    unset_timer(timer);
}
