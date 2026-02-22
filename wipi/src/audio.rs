use wipi_types::wipic::WIPICWord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unsupported,
    InvalidData,
    Platform(i32),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<wipic_sys::MediaError> for Error {
    fn from(value: wipic_sys::MediaError) -> Self {
        match value {
            wipic_sys::MediaError::Unsupported => Error::Unsupported,
            wipic_sys::MediaError::Platform(-9) => Error::InvalidData,
            wipic_sys::MediaError::Platform(code) => Error::Platform(code),
        }
    }
}

pub struct AudioClip {
    raw: WIPICWord,
}

impl AudioClip {
    pub fn from_mmf_bytes(data: &[u8]) -> Result<Self> {
        let raw = wipic_sys::media::clip_create(0, data.len() as _, 0)?;
        let written = wipic_sys::media::clip_put_data(raw, data.as_ptr() as _, data.len() as _)?;

        if written != data.len() {
            let _ = wipic_sys::media::clip_free(raw);
            return Err(Error::InvalidData);
        }

        Ok(Self { raw })
    }

    pub fn play(&self, repeat: bool) -> Result<()> {
        let repeat: WIPICWord = if repeat { 1 } else { 0 };
        wipic_sys::media::play(self.raw, repeat)?;

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        wipic_sys::media::stop(self.raw)?;

        Ok(())
    }

    pub fn pause(&self) -> Result<()> {
        wipic_sys::media::pause(self.raw)?;

        Ok(())
    }

    pub fn resume(&self) -> Result<()> {
        wipic_sys::media::resume(self.raw)?;

        Ok(())
    }

    pub fn set_volume(&self, level: u8) -> Result<()> {
        wipic_sys::media::clip_set_volume(self.raw, level.min(100) as _)?;

        Ok(())
    }

    pub fn volume(&self) -> Result<u8> {
        wipic_sys::media::clip_get_volume(self.raw).map_err(Into::into)
    }
}

impl Drop for AudioClip {
    fn drop(&mut self) {
        let _ = wipic_sys::media::clip_free(self.raw);
    }
}
