#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaError {
    Unsupported,
    Platform(i32),
}

/// Drawable area of a framebuffer, resolved from the platform's own record layout.
pub struct FramebufferInfo {
    pub width: u32,
    pub height: u32,
    pub bpl: u32,
    pub bpp: u32,
    pub pixels: *mut u8,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ktf")] {
        mod ktf;
        pub use self::ktf::*;
    } else if #[cfg(feature = "lgt")] {
        mod lgt;
        pub use self::lgt::*;
    } else if #[cfg(feature = "simulation")] {
        mod simulation;
        pub use self::simulation::*;
    }
}
