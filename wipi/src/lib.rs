#![no_std]

mod console;
mod panic_handler;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::Console, $($arg)*);
        let _ = write!($crate::Console, "\n");
    }};
}

// re-exports
pub use self::console::Console;
pub use wipic_sys::kernel::exit;
