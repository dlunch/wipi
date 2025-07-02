use wipi_types::ktf::{ExeInterface, ExeInterfaceFunctions, WipiExe};

static EXE_INTERFACE_FUNCTIONS: ExeInterfaceFunctions = ExeInterfaceFunctions {
    unk1: 0,
    unk2: 0,
    fn_init: exe_start,
    fn_get_default_dll: 0,
    fn_get_class: 0,
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

#[unsafe(no_mangle)]
unsafe extern "C" fn start() -> *const WipiExe {
    &WIPI_EXE
}

#[allow(dead_code)]
unsafe extern "C" {
    #[link_name = "startClet"]
    fn start_clet();
    #[link_name = "destroyClet"]
    fn destroy_clet();
    #[link_name = "paintClet"]
    fn paint_clet();
    #[link_name = "pauseClet"]
    fn pause_clet();
    #[link_name = "resumeClet"]
    fn resume_clet();

}

extern "C" fn wipi_start() -> u32 {
    unsafe {
        start_clet();
    } // TODO should be called by clet class

    0
}

extern "C" fn exe_start(
    _param0: u32,
    _param1: u32,
    _param2: u32,
    _param3: u32,
    _param4: u32,
) -> u32 {
    0
}
