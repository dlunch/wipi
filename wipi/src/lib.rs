#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
#[cfg(feature = "ktf")]
mod ktf;

#[cfg(target_os = "none")]
#[cfg(feature = "ktf")]
use self::ktf::wipic;

#[cfg(not(target_os = "none"))]
mod emulation;

#[cfg(not(target_os = "none"))]
use self::emulation::wipic;

unsafe extern "C" {
    #[link_name = "startClet"]
    pub fn start_clet();
    #[link_name = "destroyClet"]
    pub fn destroy_clet();
    #[link_name = "paintClet"]
    pub fn paint_clet();
    #[link_name = "pauseClet"]
    pub fn pause_clet();
    #[link_name = "resumeClet"]
    pub fn resume_clet();
}

pub fn println(s: &str) {
    wipic::println(s)
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // force crash the program
    unsafe {
        core::arch::asm!("bkpt");
    }
    loop {}
}

#[cfg(not(test))]
#[cfg(not(target_os = "none"))]
#[unsafe(no_mangle)]
pub extern "C" fn main() {
    self::emulation::start()
}
