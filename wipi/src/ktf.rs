mod clet;
mod globals;
mod java;
mod start;
pub mod wipic;

core::arch::global_asm!(include_str!("ktf/entry.s"));
