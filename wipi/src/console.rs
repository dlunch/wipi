use core::fmt;

pub struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        wipic_sys::kernel::printk(s, &[]);
        Ok(())
    }
}
