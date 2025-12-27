#[repr(u32)]
pub enum ImportModule {
    WIPIC = 0x1fb,
}

#[repr(u32)]
pub enum WIPICMethod {
    CletRegister = 0x03,
    Printk = 0x64,
    Exit = 0x6b,
    Alloc = 0x75,
    Free = 0x77,
}
