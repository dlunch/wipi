#![no_std]

#[cfg(target_arch = "arm")]
mod ktf;
// TODO

#[allow(unused_variables)]
pub fn print(s: &str) {
    todo!()
}

#[cfg(target_arch = "arm")]
#[panic_handler]
#[allow(unused_variables)]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // TODO
    loop {}
}
