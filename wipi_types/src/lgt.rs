pub mod java;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitStruct {
    pub unk1: u32,
    pub fn_init: u32,
    pub ptr_str_init: u32, // pointer to string "init"
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitParam1 {
    pub unk1: [u8; 512],
    pub unk2: [u8; 20],
    pub ptr_init_struct: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitParam2 {
    pub fn_get_import_table: u32,
    pub fn_get_import_function: u32,
    pub fn_unk3: u32,
    pub fn_unk4: u32,
}
