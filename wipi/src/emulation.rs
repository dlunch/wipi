pub mod wipic;

#[cfg(not(test))]
use crate::start_clet;

#[cfg(not(test))]
pub fn start() {
    unsafe { start_clet() }
}
