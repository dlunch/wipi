pub mod java;

use crate::wipic::TargetPtr;

cfg_if::cfg_if! {
    if #[cfg(target_os = "none")] {
        use core::ffi::c_char;

        type FnInitPtr = extern "C" fn() -> ();
        type StringPtr = *const c_char;
        type FnGetImportTablePtr = extern "C" fn(u32) -> u32;
        type FnGetImportFunctionPtr = extern "C" fn(u32, u32) -> extern "C" fn(...) -> u32;
    } else {
        use bytemuck::{Pod, Zeroable};

        type FnInitPtr = u32;
        type StringPtr = u32;
        type FnGetImportTablePtr = u32;
        type FnGetImportFunctionPtr = u32;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct InitStruct {
    pub unk1: u32,
    pub fn_init: FnInitPtr,
    pub ptr_str_init: StringPtr, // pointer to string "init"
}

unsafe impl Sync for InitStruct {}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct InitParam1 {
    pub unk1: [u8; 512],
    pub unk2: [u8; 20],
    pub ptr_init_struct: TargetPtr,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct InitParam2 {
    pub fn_get_import_table: FnGetImportTablePtr,
    pub fn_get_import_function: FnGetImportFunctionPtr,
    pub fn_unk3: u32,
    pub fn_unk4: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct CletFunctions {
    pub start_clet: TargetPtr,
    pub pause_clet: TargetPtr,
    pub resume_clet: TargetPtr,
    pub destroy_clet: TargetPtr,
    pub paint_clet: TargetPtr,
    pub handle_clet_event: TargetPtr,
}
