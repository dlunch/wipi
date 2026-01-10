#[cfg(not(target_os = "none"))]
use bytemuck::{Pod, Zeroable};

pub type WIPICWord = u32;
#[cfg(target_os = "none")]
pub type TargetPtr = *const ();
#[cfg(not(target_os = "none"))]
pub type TargetPtr = u32;

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct WIPICIndirectPtr(pub TargetPtr);

// MC_GrpDisplayInfo
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct WIPICDisplayInfo {
    pub bpp: WIPICWord,
    pub depth: WIPICWord,
    pub width: WIPICWord,
    pub height: WIPICWord,
    pub bpl: WIPICWord,
    pub color_type: WIPICWord,
    pub red_mask: WIPICWord,
    pub blue_mask: WIPICWord,
    pub green_mask: WIPICWord,
}

// MC_GrpFrameBuffer
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct WIPICFramebuffer {
    pub width: WIPICWord,
    pub height: WIPICWord,
    pub bpl: WIPICWord,
    pub bpp: WIPICWord,
    pub buf: WIPICIndirectPtr,
}

#[derive(Debug)]
#[repr(i32)]
pub enum WIPICError {
    Success = 0,
    Invalid = -9,
    NoSuchEntry = -12,
    InsufficientBufferSize = -18,
}

impl WIPICError {
    pub fn from_raw(raw: i32) -> Self {
        unsafe { core::mem::transmute(raw) }
    }
}
