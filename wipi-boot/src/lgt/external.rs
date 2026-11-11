use core::cell::RefCell;

use wipi_types::lgt::wipic::ImportModule;

use crate::lgt::globals;

type ExternalMethod = extern "C" fn(...) -> u32;

struct Cache(RefCell<[Option<(u32, u32, ExternalMethod)>; 64]>);

// Clet callbacks all run on the main thread.
unsafe impl Sync for Cache {}

static CACHE: Cache = Cache(RefCell::new([None; 64]));

pub fn get_external_method(table: ImportModule, index: u32) -> ExternalMethod {
    let table = table as u32;

    // The platform allocates a new entry point on every lookup, so each import is resolved once.
    for slot in CACHE.0.borrow_mut().iter_mut() {
        match slot {
            Some((cached_table, cached_index, method)) => {
                if *cached_table == table && *cached_index == index {
                    return *method;
                }
            }
            None => {
                let method = resolve(table, index);
                *slot = Some((table, index, method));
                return method;
            }
        }
    }

    resolve(table, index)
}

fn resolve(table: u32, index: u32) -> ExternalMethod {
    unsafe {
        let table = ((*globals::INIT_PARAM_2).fn_get_import_table)(table);
        ((*globals::INIT_PARAM_2).fn_get_import_function)(table as _, index)
    }
}
