#![no_std]
#![no_main]

use wipi::print;

#[unsafe(no_mangle)]
extern "C" fn main() {
    print("Hello, world!");
}
