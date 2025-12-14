use crate::lgt::globals;

pub fn get_external_method(table: u32, index: u32) -> extern "C" fn(...) -> u32 {
    // TODO caching
    unsafe {
        let table = ((*globals::INIT_PARAM_2).fn_get_import_table)(table);
        ((*globals::INIT_PARAM_2).fn_get_import_function)(table, index)
    }
}
