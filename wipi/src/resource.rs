use alloc::ffi::CString;

use wipi_types::wipic::{WIPICError, WIPICIndirectPtr};
use wipic_sys::deref_indirect_ptr;

use crate::Result;

pub struct Resource {
    size: usize,
    buf: WIPICIndirectPtr,
}

impl Resource {
    pub fn new(path: &str) -> Result<Self> {
        let mut size = 0;
        let path = CString::new(path).unwrap();
        let result =
            unsafe { wipic_sys::kernel::get_resource_id(path.as_ptr(), &mut size as *mut _) };
        if result < 0 {
            return Err(WIPICError::from_raw(result));
        }

        let buf = wipic_sys::kernel::alloc(size as _);
        wipic_sys::kernel::get_resource(result, buf, size);

        Ok(Self { buf, size })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn read(&self) -> &[u8] {
        unsafe {
            let buf_ptr = deref_indirect_ptr(self.buf);
            core::slice::from_raw_parts(buf_ptr as _, self.size)
        }
    }

    pub fn buf_raw(&self) -> WIPICIndirectPtr {
        self.buf
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        wipic_sys::kernel::free(self.buf);
    }
}
