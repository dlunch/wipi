use core::ffi::c_char;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExeInterfaceFunctions {
    pub unk1: u32,
    pub unk2: u32,
    pub fn_init: unsafe extern "C" fn(u32, u32, u32, u32, u32) -> u32,
    pub fn_get_default_dll: u32,
    pub fn_get_class: u32,
    pub fn_unk2: u32,
    pub fn_unk3: u32,
}

unsafe impl Sync for ExeInterfaceFunctions {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExeInterface {
    pub ptr_functions: *const ExeInterfaceFunctions,
    pub ptr_name: *const c_char,
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
pub struct WipiExe {
    pub ptr_exe_interface: *const ExeInterface,
    pub ptr_name: *const c_char,
    pub unk1: u32,
    pub unk2: u32,
    pub fn_unk1: u32,
    pub fn_init: unsafe extern "C" fn() -> u32,
    pub unk3: u32,
    pub unk4: u32,
    pub fn_unk3: u32,
    pub unk5: u32,
}

unsafe impl Sync for WipiExe {}
