use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitStruct {
    unk1: u32,
    fn_init: u32,
    ptr_str_init: u32, // pointer to string "init"
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitParam1 {
    unk1: [u8; 512],
    unk2: [u8; 20],
    ptr_init_struct: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitParam2 {
    fn_get_import_table: u32,
    fn_get_import_function: u32,
    fn_unk3: u32,
    fn_unk4: u32,
}
