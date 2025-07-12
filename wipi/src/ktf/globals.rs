use core::ptr::null;

use wipi_types::ktf::{InitParam0, InitParam1, InitParam2, InitParam3, InitParam4};

pub(super) static mut INIT_PARAM_0: *const InitParam0 = null();
pub(super) static mut INIT_PARAM_1: *const InitParam1 = null();
pub(super) static mut INIT_PARAM_2: *const InitParam2 = null();
pub(super) static mut INIT_PARAM_3: *const InitParam3 = null();
pub(super) static mut INIT_PARAM_4: *const InitParam4 = null();
