#[cfg(not(target_os = "none"))]
use bytemuck::{Pod, Zeroable};

pub type WIPICWord = u32;
#[cfg(target_os = "none")]
pub type TargetPtr = *const ();
#[cfg(not(target_os = "none"))]
pub type TargetPtr = u32;

#[repr(C)]
#[derive(Default, Clone, Copy)]
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

// MC_GrpImage
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct WIPICImage {
    pub img: WIPICFramebuffer,
    pub mask: WIPICFramebuffer,
    pub loop_count: WIPICWord,
    pub delay: WIPICWord,
    pub animated: WIPICWord,
    pub buf: WIPICIndirectPtr,
    pub offset: WIPICWord,
    pub current: WIPICWord,
    pub len: WIPICWord,
}

// MC_GrpContext
#[repr(C)]
#[derive(Default, Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct WIPICGraphicsContext {
    pub mask: WIPICWord,
    /// top-left x, y, bottom-right x, y
    pub clip: [WIPICWord; 4],
    pub fgpxl: WIPICWord,
    pub bgpxl: WIPICWord,
    pub transpxl: WIPICWord,
    pub alpha: WIPICWord,
    /// x, y
    pub offset: [WIPICWord; 2],
    pub pixel_op_func_ptr: WIPICWord, // MC_GrpPixelOpProc
    pub param1: WIPICWord,
    pub reserved: WIPICWord,
    pub font: WIPICWord,
    pub style: WIPICWord,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum WIPICError {
    ImageDone = 1,
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
