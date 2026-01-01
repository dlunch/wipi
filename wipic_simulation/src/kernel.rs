use std::{println, process};

pub fn printk(fmt: &str, _args: &[*const ()]) {
    println!("{fmt}");
}

pub fn exit(code: i32) {
    process::exit(code);
}
