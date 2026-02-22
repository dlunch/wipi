use wipi_types::wipic::{WIPICError, WIPICImage, WIPICIndirectPtr};
use wipic_sys::deref_indirect_ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Resource(super::resource::Error),
    InvalidData,
    Platform(i32),
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct Image {
    raw: WIPICIndirectPtr,
}

impl Image {
    pub fn new(path: &str) -> Result<Self> {
        let resource = crate::resource::Resource::new(path).map_err(Error::Resource)?;

        let mut raw = WIPICIndirectPtr::default();
        let result = unsafe {
            wipic_sys::graphics::create_image(
                &mut raw as *mut _,
                resource.buf_raw(),
                0,
                resource.size() as u32,
            )
        };
        if result == WIPICError::Invalid {
            return Err(Error::InvalidData);
        }
        if result != WIPICError::ImageDone {
            return Err(Error::Platform(result as i32));
        }

        Ok(Self { raw })
    }

    fn read_image(&self) -> &WIPICImage {
        unsafe { &*(deref_indirect_ptr(self.raw) as *const _) }
    }

    pub fn raw(&self) -> WIPICIndirectPtr {
        self.raw
    }

    pub fn width(&self) -> u32 {
        self.read_image().img.width as _
    }

    pub fn height(&self) -> u32 {
        self.read_image().img.height as _
    }
}
