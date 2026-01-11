#[cfg(not(feature = "simulation"))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    crate::println!("{}", info);

    // force crash the program
    unsafe {
        core::arch::asm!("bkpt");
    }
    loop {}
}
