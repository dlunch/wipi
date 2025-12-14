extern crate std;

use std::{println, process};

pub fn println(s: &str) {
    println!("{s}");
}

pub fn exit(code: i32) {
    process::exit(code);
}
