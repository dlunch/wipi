#![no_std]

#[cfg(target_os = "none")]
#[cfg(feature = "ktf")]
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

#[cfg(not(target_os = "none"))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[cfg(target_os = "linux")]
#[unsafe(export_name = "__libc_start_main")]
pub fn main() {}
