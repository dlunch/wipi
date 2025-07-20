use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClass {
    pub unk1: u32,
    pub unk2: u32,
    pub ptr_descriptor: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClassDescriptor {
    pub unk1: u32,
    pub unk2: u32,
    pub ptr_name: u32,
    pub unk3: u32,
    pub ptr_super_class_name: u32,
    pub unk4: u32,
    pub unk5: u16,
    pub unk6: u16,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u8,
    pub unk10: u8,
    pub unk11: u16,
    pub unk12: u32,
    pub fn_unk1: u32,
    pub fn_unk2: u32,
    pub fn_unk3: u32,
    pub ptr_methods: u32,
    pub ptr_fields: u32,
    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClassField {
    pub ptr_class: u32,
    pub ptr_name: u32,
    pub ptr_type: u32,
    pub unk1: u16,
    pub unk2: u16,
    pub index: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClassFields {
    pub count: u32,
    pub fields: [LgtJavaClassField; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClassMethod {
    pub ptr_class: u32,
    pub ptr_name: u32,
    pub ptr_type: u32,
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u32,
    pub ptr_method: u32,
    pub unk4: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClassMethods {
    pub count: u32,
    pub methods: [LgtJavaClassMethod; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaImportedClass {
    pub ptr_name: u32,
    pub unk1: u32,
    pub static_field_offset: u16,
    pub static_field_count: u16,
    pub virtual_method_offset: u16,
    pub virtual_method_count: u16,
    pub unk2: u32,
    pub static_method_offset: u16,
    pub static_method_count: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaImportedClasses {
    pub count: u32,
    pub classes: [LgtJavaImportedClass; 0],
}

#[repr(C)]
pub struct LgtJavaClassInstance {
    pub ptr_vtable: u32,
    pub unk1: u32,
    pub ptr_fields: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaPublicClass {
    pub ptr_name: u32,
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u32,
    pub virtual_method_offset: u16,
    pub virtual_method_count: u16,
    pub unk4: u32,
    pub unk5: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaPublicClasses {
    pub count: u32,
    pub classes: [LgtJavaPublicClass; 0],
}
