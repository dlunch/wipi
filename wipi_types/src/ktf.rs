#[cfg(target_os = "none")]
use core::ffi::c_char;

#[cfg(target_os = "none")]
type ExeInterfaceInitPtr = unsafe extern "C" fn(u32, u32, u32, u32, u32) -> u32;
#[cfg(not(target_os = "none"))]
type ExeInterfaceInitPtr = u32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExeInterfaceFunctions {
    pub unk1: u32,
    pub unk2: u32,
    pub fn_init: ExeInterfaceInitPtr,
    pub fn_get_default_dll: u32,
    pub fn_get_class: u32,
    pub fn_unk2: u32,
    pub fn_unk3: u32,
}

unsafe impl Sync for ExeInterfaceFunctions {}

#[cfg(target_os = "none")]
type ExeInterfaceFunctionsPtr = *const ExeInterfaceFunctions;
#[cfg(not(target_os = "none"))]
type ExeInterfaceFunctionsPtr = u32;

#[cfg(target_os = "none")]
type ExeInterfaceNamePtr = *const c_char;
#[cfg(not(target_os = "none"))]
type ExeInterfaceNamePtr = u32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExeInterface {
    pub ptr_functions: ExeInterfaceFunctionsPtr,
    pub ptr_name: ExeInterfaceNamePtr,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}

unsafe impl Sync for ExeInterface {}

#[cfg(target_os = "none")]
type ExeInterfacePtr = *const ExeInterface;
#[cfg(not(target_os = "none"))]
type ExeInterfacePtr = u32;

#[cfg(target_os = "none")]
type WipiExeNamePtr = *const c_char;
#[cfg(not(target_os = "none"))]
type WipiExeNamePtr = u32;

#[cfg(target_os = "none")]
type WipiExeInitPtr = unsafe extern "C" fn() -> u32;
#[cfg(not(target_os = "none"))]
type WipiExeInitPtr = u32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WipiExe {
    pub ptr_exe_interface: ExeInterfacePtr,
    pub ptr_name: WipiExeNamePtr,
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
