mod external;
mod globals;
mod start;
pub mod wipic;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // force crash the program
    unsafe {
        core::arch::asm!("bkpt");
    }
    loop {}
}
