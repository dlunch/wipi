use core::ffi::c_char;

use wipi_types::ktf::{
    ExeInterface, ExeInterfaceFunctions, InitParam0, InitParam1, InitParam2, InitParam3,
    InitParam4, WipiExe,
};

static EXE_INTERFACE_FUNCTIONS: ExeInterfaceFunctions = ExeInterfaceFunctions {
    unk1: 0,
    unk2: 0,
    fn_init: exe_start,
    fn_get_default_dll: 0,
    fn_get_class: get_class,
    fn_unk2: 0,
    fn_unk3: 0,
};

static EXE_INTERFACE: ExeInterface = ExeInterface {
    ptr_functions: &EXE_INTERFACE_FUNCTIONS,
    ptr_name: c"ExeInterface".as_ptr(),
    unk1: 0,
    unk2: 0,
    unk3: 0,
    unk4: 0,
    unk5: 0,
    unk6: 0,
};

static WIPI_EXE: WipiExe = WipiExe {
    ptr_exe_interface: &EXE_INTERFACE,
    ptr_name: c"WIPI_exe".as_ptr(),
    unk1: 0,
    unk2: 0,
    fn_unk1: 0,
    fn_init: wipi_start,
    unk3: 0,
    unk4: 0,
    fn_unk3: 0,
    unk5: 0,
};

pub(super) static mut INIT_PARAM_0: *const InitParam0 = core::ptr::null_mut();
pub(super) static mut INIT_PARAM_1: *const InitParam1 = core::ptr::null_mut();
pub(super) static mut INIT_PARAM_2: *const InitParam2 = core::ptr::null_mut();
pub(super) static mut INIT_PARAM_3: *const InitParam3 = core::ptr::null_mut();
pub(super) static mut INIT_PARAM_4: *const InitParam4 = core::ptr::null_mut();

#[unsafe(no_mangle)]
unsafe extern "C" fn start() -> *const WipiExe {
    // TODO should perform relocations
    &WIPI_EXE
}

extern "C" fn get_class(_name: *const c_char) -> u32 {
    // TODO

    0
}

extern "C" fn wipi_start() -> u32 {
    // TODO should initialize java environments(instantiate java constant strings, resolve external superclasses, ...)
    0
}

extern "C" fn exe_start(
    param0: *const InitParam0,
    param1: *const InitParam1,
    param2: *const InitParam2,
    param3: *const InitParam3,
    param4: *const InitParam4,
) -> u32 {
    unsafe {
        INIT_PARAM_0 = param0;
        INIT_PARAM_1 = param1;
        INIT_PARAM_2 = param2;
        INIT_PARAM_3 = param3;
        INIT_PARAM_4 = param4;
    }

    0
}
