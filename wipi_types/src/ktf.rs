pub mod java;
pub mod kernel;

use bytemuck::{Pod, Zeroable};

cfg_if::cfg_if! {
    if #[cfg(target_os = "none")] {
        use core::ffi::c_char;

        type ExeInterfaceInitPtr = extern "C" fn(*const InitParam0, *const InitParam1, *const InitParam2, *const InitParam3, *const InitParam4) -> u32;
        type ExeInterfaceGetClassPtr = extern "C" fn(*const c_char) -> u32;
        type ExeInterfaceFunctionsPtr = *const ExeInterfaceFunctions;
        type StringPtr = *const c_char;
        type ExeInterfacePtr = *const ExeInterface;
        type WipiExeInitPtr = extern "C" fn() -> u32;
        type GetInterfacePtr = extern "C" fn(*const c_char) -> *const ();
        type TargetPtr = *const ();
    } else {
        type ExeInterfaceInitPtr = u32;
        type ExeInterfaceGetClassPtr = u32;
        type ExeInterfaceFunctionsPtr = u32;
        type StringPtr = u32;
        type ExeInterfacePtr = u32;
        type WipiExeInitPtr = u32;
        type GetInterfacePtr = u32;
        type TargetPtr = u32;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitParam0 {
    pub unk: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitParam1 {
    pub ptr_jvm_exception_context: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitParam2 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub ptr_java_vtables: [u32; 128],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitParam3 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    // java array allocation pool for primitive type
    pub boolean: u32,
    pub char: u32,
    pub float: u32,
    pub double: u32,
    pub byte: u32,
    pub short: u32,
    pub int: u32,
    pub long: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct InitParam4 {
    pub fn_get_interface: GetInterfacePtr,
    pub fn_java_throw: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub fn_java_check_type: u32,
    pub fn_java_new: u32,
    pub fn_java_array_new: u32,
    pub unk6: u32,
    pub fn_java_class_load: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub fn_alloc: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct ExeInterfaceFunctions {
    pub unk1: u32,
    pub unk2: u32,
    pub fn_init: ExeInterfaceInitPtr,
    pub fn_get_default_dll: u32,
    pub fn_get_class: ExeInterfaceGetClassPtr,
    pub fn_unk2: u32,
    pub fn_unk3: u32,
}

unsafe impl Sync for ExeInterfaceFunctions {}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct ExeInterface {
    pub ptr_functions: ExeInterfaceFunctionsPtr,
    pub ptr_name: StringPtr,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}

unsafe impl Sync for ExeInterface {}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct WipiExe {
    pub ptr_exe_interface: ExeInterfacePtr,
    pub ptr_name: StringPtr,
    pub unk1: u32,
    pub unk2: u32,
    pub fn_unk1: u32,
    pub fn_init: WipiExeInitPtr,
    pub unk3: u32,
    pub unk4: u32,
    pub fn_unk3: u32,
    pub unk5: u32,
}

unsafe impl Sync for WipiExe {}
