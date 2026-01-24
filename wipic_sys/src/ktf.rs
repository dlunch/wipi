pub mod database;
pub mod graphics;
pub mod kernel;

use wipi_types::wipic::WIPICIndirectPtr;

pub fn deref_indirect_ptr(ptr: WIPICIndirectPtr) -> *mut u8 {
    unsafe {
        let base = *(ptr.0 as *const u32);

        (base as *mut u8).add(8)
    }
}

/// # Safety
/// it's caller responsibility to ensure the pointer is valid
pub unsafe fn to_indirect_ptr(ptr: *mut u8) -> WIPICIndirectPtr {
    // HACK: wie specific layout
    WIPICIndirectPtr(unsafe { ptr.sub(12) } as _)
}
