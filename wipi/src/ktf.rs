mod clet;
mod clet_card;
mod globals;
mod java;
mod start;
pub mod wipic;

use crate::println;

core::arch::global_asm!(include_str!("ktf/entry.s"));

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println(info.message().as_str().unwrap_or("Panic occurred"));

    // force crash the program
    unsafe {
        core::arch::asm!("bkpt");
    }
    loop {}
}
