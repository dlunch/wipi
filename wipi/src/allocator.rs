use alloc::alloc::GlobalAlloc;

use wipic_sys::{deref_indirect_ptr, to_indirect_ptr};

pub struct WipiAllocator;

unsafe impl GlobalAlloc for WipiAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        // alignment is ignored for now
        let ptr = wipic_sys::kernel::alloc(layout.size() as _);

        deref_indirect_ptr(&ptr)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        let indirect_ptr = unsafe { to_indirect_ptr(ptr) };
        wipic_sys::kernel::free(indirect_ptr);
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: WipiAllocator = WipiAllocator;
