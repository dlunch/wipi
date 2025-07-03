cfg_if::cfg_if! {
    if #[cfg(target_os = "none")] {
        use core::ffi::c_char;

        type ExeInterfaceInitPtr = unsafe extern "C" fn(u32, u32, u32, u32, u32) -> u32;
        type ExeInterfaceFunctionsPtr = *const ExeInterfaceFunctions;
        type ExeInterfaceNamePtr = *const c_char;
        type ExeInterfacePtr = *const ExeInterface;
        type WipiExeNamePtr = *const c_char;
        type WipiExeInitPtr = unsafe extern "C" fn() -> u32;
    } else {
        use bytemuck::{Pod, Zeroable};

        type ExeInterfaceInitPtr = u32;
        type ExeInterfaceFunctionsPtr = u32;
        type ExeInterfaceNamePtr = u32;
        type ExeInterfacePtr = u32;
        type WipiExeNamePtr = u32;
        type WipiExeInitPtr = u32;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
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

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
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

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
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
