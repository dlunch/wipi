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
    GetResourceId = 0x80,
    GetResource = 0x81,
    GetScreenFramebuffer = 0xca,
    InitContext = 0xcd,
    DrawRect = 0xd2,
    FillRect = 0xd3,
    DrawImage = 0xd5,
    DrawString = 0xda,
    FlushLcd = 0xde,
    Repaint = 0xe2,
    CreateImage = 0xe9,
    OpenDatabase = 0x190,
    ReadRecordSingle = 0x191,
    WriteRecordSingle = 0x192,
    CloseDatabase = 0x193,
}
