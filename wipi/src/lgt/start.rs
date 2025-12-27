use core::mem::transmute;

use wipi_types::lgt::{
    wipic::{ImportModule, WIPICMethod},
    {CletFunctions, InitParam1, InitParam2, InitStruct},
};

use crate::{
    destroy_clet, handle_clet_event,
    lgt::{external::get_external_method, globals},
    paint_clet, pause_clet, resume_clet, start_clet,
};

static INIT_STRUCT: InitStruct = InitStruct {
    unk1: 0,
    fn_init: init,
    ptr_str_init: c"init".as_ptr(),
};

#[unsafe(export_name = "_start")]
pub fn start(init_param_1: *mut InitParam1, init_param_2: *mut InitParam2, _unk: u32) {
    unsafe {
        (*init_param_1).ptr_init_struct = &INIT_STRUCT as *const _ as _;

        globals::INIT_PARAM_2 = init_param_2;
    };
}

extern "C" fn init() {
    // TODO clet only for now
    let functions = CletFunctions {
        start_clet: start_clet as _,
        pause_clet: pause_clet as _,
        resume_clet: resume_clet as _,
        destroy_clet: destroy_clet as _,
        paint_clet: paint_clet as _,
        handle_clet_event: handle_clet_event as _,
    };

    unsafe {
        // TODO enum
        let clet_register: extern "C" fn(*const CletFunctions, u32) =
            transmute(get_external_method(ImportModule::WIPIC, WIPICMethod::CletRegister as _));

        clet_register(&functions as *const _, 0)
    };
}
