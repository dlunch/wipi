#![no_std]

#[cfg(target_os = "none")]
mod ktf;
// TODO

#[allow(unused_variables)]
pub fn print(s: &str) {
    todo!()
}

#[cfg(target_os = "none")]
#[panic_handler]
#[allow(unused_variables)]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // TODO
    loop {}
}
