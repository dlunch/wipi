#[cfg(not(target_os = "none"))]
use bytemuck::{Pod, Zeroable};

pub type WIPICWord = u32;

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct WIPICIndirectPtr(pub WIPICWord);

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
