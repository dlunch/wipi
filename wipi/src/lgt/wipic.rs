pub fn println(_s: &str) {
    todo!()
}

pub fn exit(_code: i32) {
    todo!()
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // force crash the program
    unsafe {
        core::arch::asm!("bkpt");
    }
    loop {}
}
