#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
#[cfg(feature = "ktf")]
mod ktf;
// TODO

pub fn print(_s: &str) {
    todo!()
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
#[unsafe(no_mangle)]
pub extern "C" fn main() {} // we need main to link on the host side
