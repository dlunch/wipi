use core::ptr;

use wipi_types::lgt::InitParam2;

pub(super) static mut INIT_PARAM_2: *const InitParam2 = ptr::null();
