#![no_std]
extern crate alloc;

#[cfg(not(feature = "simulation"))]
mod allocator;
pub mod app;
pub mod console;
pub mod database;
pub mod event;
pub mod framebuffer;
pub mod image;
#[cfg(not(test))]
mod lifecycle;
mod panic_handler;
pub mod resource;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::console::Console, $($arg)*);
        let _ = write!($crate::console::Console, "\n");
    }};
}

pub type Result<T> = core::result::Result<T, wipi_types::wipic::WIPICError>;

// re-exports
pub mod kernel {
    pub use wipic_sys::kernel::exit;
}
pub use wipi_macros::wipi_main;
#[doc(hidden)]
pub mod __internal {
    #[cfg(feature = "simulation")]
    pub use wipi_boot::simulation::simulation_start;
}
