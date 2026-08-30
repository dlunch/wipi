use wipi_types::lgt::wipic::ImportModule;

use crate::lgt::globals;

pub fn get_external_method(table: ImportModule, index: u32) -> extern "C" fn(...) -> u32 {
    // TODO caching
    unsafe {
        let table = ((*globals::INIT_PARAM_2).fn_get_import_table)(table as _);
        ((*globals::INIT_PARAM_2).fn_get_import_function)(table as _, index)
    }
}
