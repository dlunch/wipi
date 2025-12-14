pub mod graphics;
pub mod kernel;

#[repr(transparent)]
pub struct KtfWipiCMemory(*mut u8);

impl KtfWipiCMemory {
    pub fn as_ptr(&self) -> *mut u8 {
        unsafe { self.0.add(12) }
    }
}
