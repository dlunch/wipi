use core::{
    ffi::{CStr, c_char},
    mem::MaybeUninit,
    ptr,
};

use wipi_types::ktf::{
    ExeInterface, ExeInterfaceFunctions, InitParam0, InitParam1, InitParam2, InitParam3,
    InitParam4, WipiExe, java::JavaClass,
};

use crate::ktf::{clet::CLET_CLASS, clet_card::CLET_CARD_CLASS, globals};

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

#[unsafe(no_mangle)]
unsafe extern "C" fn start() -> *const WipiExe {
    // TODO should perform relocations
    &WIPI_EXE
}

extern "C" fn get_class(name: *const c_char) -> *const () {
    // TODO Clet only for now

    unsafe {
        if CStr::from_ptr(name) == c"Clet" {
            return &raw const CLET_CLASS as *const ();
        } else if CStr::from_ptr(name) == c"CletCard" {
            return &raw const CLET_CARD_CLASS as *const ();
        }
    }

    ptr::null()
}

extern "C" fn wipi_start() -> u32 {
    // TODO instantiate java static strings

    // resolve superclasses
    unsafe {
        // TODO should discover classes by some kind of table
        let classes = [&raw mut CLET_CLASS, &raw mut CLET_CARD_CLASS];

        for class in classes {
            let super_class = (*(*class).ptr_descriptor).ptr_parent_class as *const c_char;

            let mut ptr_super_class = MaybeUninit::<*const JavaClass>::uninit();
            let result = ((*globals::INIT_PARAM_4).fn_java_class_load)(
                ptr_super_class.as_mut_ptr(),
                super_class,
            );

            let ptr_super_class = ptr_super_class.assume_init();

            if result != 0 {
                // TODO error handling
                panic!("Can't load class");
            }

            (*(*class).ptr_descriptor).ptr_parent_class = ptr_super_class as _;
            (*(*class).ptr_descriptor).fields_size +=
                (*(*ptr_super_class).ptr_descriptor).fields_size;
        }
    }

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
        globals::INIT_PARAM_0 = param0;
        globals::INIT_PARAM_1 = param1;
        globals::INIT_PARAM_2 = param2;
        globals::INIT_PARAM_3 = param3;
        globals::INIT_PARAM_4 = param4;

        globals::WIPIC_KNLINTERFACE =
            ((*param4).fn_get_interface)(c"WIPIC_knlInterface".as_ptr()) as _;
        globals::WIPI_JBINTERFACE = ((*param4).fn_get_interface)(c"WIPI_JBInterface".as_ptr()) as _;
    }

    0
}
