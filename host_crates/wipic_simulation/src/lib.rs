pub mod graphics;
pub mod kernel;

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn main() {
    unsafe { wipi_boot::start_clet() }
    kernel::exit(0);
}
