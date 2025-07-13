mod clet;
mod globals;
mod java;
mod start;
pub mod wipic;

core::arch::global_asm!(include_str!("ktf/entry.s"));

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // force crash the program
    unsafe {
        core::arch::asm!("bkpt");
    }
    loop {}
}
