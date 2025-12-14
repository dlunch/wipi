use crate::wipic::kernel::println;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println(info.message().as_str().unwrap_or("Panic occurred"));

    // force crash the program
    unsafe {
        core::arch::asm!("bkpt");
    }
    loop {}
}
