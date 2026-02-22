use alloc::ffi::CString;

use wipi_types::wipic::{WIPICError, WIPICIndirectPtr};
use wipic_sys::deref_indirect_ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidPath,
    Platform(i32),
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct Resource {
    size: usize,
    buf: WIPICIndirectPtr,
}

impl Resource {
    pub fn new(path: &str) -> Result<Self> {
        let mut size = 0;
        let path = CString::new(path).map_err(|_| Error::InvalidPath)?;
        let result =
            unsafe { wipic_sys::kernel::get_resource_id(path.as_ptr(), &mut size as *mut _) };
        if result < 0 {
            return Err(Error::Platform(result));
        }

        let buf = wipic_sys::kernel::alloc(size as _);
        let resource_result = wipic_sys::kernel::get_resource(result, buf, size);
        if resource_result != WIPICError::Success {
            wipic_sys::kernel::free(buf);
            return Err(Error::Platform(resource_result as i32));
        }

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
