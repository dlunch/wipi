use core::ptr;

use wipi_types::ktf::{
    InitParam0, InitParam1, InitParam2, InitParam3, InitParam4,
    java::WIPIJBInterface,
    wipic::{WIPICGraphicsInterface, WIPICKnlInterface},
};

pub(super) static mut INIT_PARAM_0: *const InitParam0 = ptr::null();
pub(super) static mut INIT_PARAM_1: *const InitParam1 = ptr::null();
pub(super) static mut INIT_PARAM_2: *const InitParam2 = ptr::null();
pub(super) static mut INIT_PARAM_3: *const InitParam3 = ptr::null();
pub(super) static mut INIT_PARAM_4: *const InitParam4 = ptr::null();
pub(super) static mut WIPI_JBINTERFACE: *const WIPIJBInterface = ptr::null();

// for wipic
pub static mut WIPIC_KNLINTERFACE: *const WIPICKnlInterface = ptr::null();
pub static mut WIPIC_GRAPHICS_INTERFACE: *const WIPICGraphicsInterface = ptr::null();
