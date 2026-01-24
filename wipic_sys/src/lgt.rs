pub mod database;
pub mod graphics;
pub mod kernel;

use wipi_types::wipic::WIPICIndirectPtr;

pub fn deref_indirect_ptr(ptr: WIPICIndirectPtr) -> *mut u8 {
    ptr.0 as *mut u8
}

/// # Safety
/// Always safe to call
pub unsafe fn to_indirect_ptr(ptr: *mut u8) -> WIPICIndirectPtr {
    WIPICIndirectPtr(ptr as _)
}
