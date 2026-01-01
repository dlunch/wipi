#[cfg(target_os = "none")]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    wipic_sys::kernel::printk(info.message().as_str().unwrap_or("Panic occurred"), &[]);

    // force crash the program
    unsafe {
        core::arch::asm!("bkpt");
    }
    loop {}
}
