#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaError {
    Unsupported,
    Platform(i32),
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
