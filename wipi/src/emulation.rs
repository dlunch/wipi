pub mod wipic;

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn main() {
    unsafe { crate::start_clet() }
}
