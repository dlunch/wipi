#[cfg(not(target_os = "none"))]
use bytemuck::{Pod, Zeroable};

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
    DefTimer = 0x7a,
    SetTimer = 0x7b,
    UnsetTimer = 0x7c,
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
    ClipCreate = 0x4b0,
    ClipFree = 0x4b1,
    ClipPutData = 0x4b3,
    ClipGetVolume = 0x4b8,
    ClipSetVolume = 0x4b9,
    Play = 0x4ba,
    Stop = 0x4bd,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct LgtFramebuffer {
    pub owned_image: u32,
    pub ptr_graphics: u32,
    pub ptr_image: u32,
    pub screen_kind: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct LgtGraphicsView {
    pub ptr_backing: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct LgtImage {
    pub ptr_image: u32,
    pub ptr_framebuffer: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct LgtGraphicsContext {
    pub clip_x1: i32,
    pub clip_y1: i32,
    pub clip_x2: i32,
    pub clip_y2: i32,
    pub foreground: u32,
    pub background: u32,
    pub alpha: u32,
    pub pixel_op: u32,
    pub pixel_param: u32,
    pub font: u32,
    pub style: u32,
    pub xor_enabled: u32,
    pub offset_x: i32,
    pub offset_y: i32,
}
