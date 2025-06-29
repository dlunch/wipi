use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target == "thumbv4t-none-eabi" {
        println!("cargo::rerun-if-changed=src/ktf.ld");
    }
}
