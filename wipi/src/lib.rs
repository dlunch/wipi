#![no_std]

mod ktf;
// TODO

#[allow(unused_variables)]
pub fn print(s: &str) {
    todo!()
}

#[panic_handler]
#[allow(unused_variables)]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // TODO
    loop {}
}
