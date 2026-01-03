use std::{print, process};

pub fn printk(fmt: &str, _args: &[*const ()]) {
    print!("{fmt}");
}

pub fn exit(code: i32) {
    process::exit(code);
}
