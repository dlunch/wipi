mod clet;
mod globals;
mod java;
mod start;

core::arch::global_asm!(include_str!("ktf/entry.s"));
