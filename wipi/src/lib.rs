#![no_std]
extern crate alloc;

#[cfg(target_os = "none")]
mod allocator;
pub mod app;
pub mod console;
pub mod event;
pub mod framebuffer;
mod panic_handler;
#[cfg(not(test))]
mod sys;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::console::Console, $($arg)*);
        let _ = write!($crate::console::Console, "\n");
    }};
}

// re-exports
pub mod kernel {
    pub use wipic_sys::kernel::exit;
}
pub use wipi_macros::wipi_main;
