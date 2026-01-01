mod clet;
mod clet_card;
mod globals;
mod java;
mod start;

core::arch::global_asm!(include_str!("ktf/entry.s"));

pub use globals::{WIPIC_GRAPHICS_INTERFACE, WIPIC_KNLINTERFACE};
