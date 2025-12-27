pub mod graphics;
pub mod kernel;

use wipi_types::wipic::WIPICIndirectPtr;

pub fn deref_indirect_ptr(ptr: WIPICIndirectPtr) -> *mut u8 {
    unsafe { (ptr.0 as *mut u8).add(8) } // Offset by 8 bytes to get the actual pointer
}
