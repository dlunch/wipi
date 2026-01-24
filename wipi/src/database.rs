use wipi_types::wipic::WIPICError;

pub struct Database {
    handle: i32,
}

impl Database {
    pub fn open(name: &str, mode: OpenMode) -> Result<Self, WIPICError> {
        let name_bytes = name.as_bytes();
        let mut name_buf = [0u8; 32];
        name_buf[..name_bytes.len()].copy_from_slice(name_bytes);

        let handle =
            unsafe { wipic_sys::database::open_database(name_buf.as_ptr(), mode as i32, 0) };
        if handle < 0 {
            return Err(WIPICError::from_raw(handle));
        }

        Ok(Self { handle })
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, WIPICError> {
        let result = unsafe {
            wipic_sys::database::read_record_single(self.handle, buf.as_mut_ptr(), buf.len() as u32)
        };
        if result < 0 {
            return Err(WIPICError::from_raw(result));
        }
        Ok(result as usize)
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, WIPICError> {
        let result = unsafe {
            wipic_sys::database::write_record_single(self.handle, data.as_ptr(), data.len() as u32)
        };
        if result < 0 {
            return Err(WIPICError::from_raw(result));
        }
        Ok(result as usize)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        wipic_sys::database::close_database(self.handle);
    }
}

#[repr(i32)]
pub enum OpenMode {
    ReadOnly = 1,
    ReadWrite = 0,
}
