use alloc::ffi::CString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidName,
    Platform(i32),
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct File {
    fd: i32,
}

impl File {
    pub fn open(name: &str, mode: OpenMode) -> Result<Self> {
        let name = CString::new(name).map_err(|_| Error::InvalidName)?;

        let fd = unsafe { wipic_sys::fs::open(name.as_ptr().cast(), mode as i32, 1) };
        if fd < 0 {
            return Err(Error::Platform(fd));
        }

        Ok(Self { fd })
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = i32::try_from(buf.len()).map_err(|_| Error::Platform(-1))?;
        let result = unsafe { wipic_sys::fs::read(self.fd, buf.as_mut_ptr(), len) };
        if result < 0 {
            return Err(Error::Platform(result));
        }
        Ok(result as usize)
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        let len = i32::try_from(data.len()).map_err(|_| Error::Platform(-1))?;
        let result = unsafe { wipic_sys::fs::write(self.fd, data.as_ptr(), len) };
        if result < 0 {
            return Err(Error::Platform(result));
        }
        Ok(result as usize)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        wipic_sys::fs::close(self.fd);
    }
}

#[repr(i32)]
pub enum OpenMode {
    ReadOnly = 1,
    WriteOnly = 2,
    WriteTruncate = 4,
    ReadWrite = 8,
}
