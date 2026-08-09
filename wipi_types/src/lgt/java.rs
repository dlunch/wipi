use bytemuck::{Pod, Zeroable};

pub const LGT_JAVA_CLASS_SUPER_CLASS_IS_NAME: u8 = 0x02;

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
    pub access_flags: u32,
    /// Next class record in the compiler-generated singly linked list.
    pub ptr_next_class: u32,
    pub ptr_name: u32,
    /// Compiler-generated dispatch table with the class pointer in word zero.
    pub ptr_vtable: u32,
    /// Points to a class record, or to its name when `LGT_JAVA_CLASS_SUPER_CLASS_IS_NAME` is set in `flags`.
    pub ptr_super_class: u32,
    pub unk4: u32,
    /// Total 32-bit instance field words, including words inherited from the superclass.
    pub instance_field_word_count: u16,
    /// Registration state checked before resolving the runtime class.
    pub link_state: u16,
    pub unk7: u32,
    /// Bitmap of object-reference instance field words, ordered by word index and most-significant bit first within each byte.
    pub ptr_instance_reference_bitmap: u32,
    pub flags: u8,
    pub unk10: u8,
    /// Number of method entries following the class pointer in `ptr_vtable`.
    pub vtable_count: u16,
    /// Points to a count-prefixed `LgtJavaClassNames` record.
    pub ptr_interface_names: u32,
    /// Links member metadata to a runtime class and patches member indices.
    pub fn_link_members: u32,
    /// Resolves the runtime class and ensures its `<clinit>` callback has run.
    pub fn_get_initialized_class: u32,
    /// Registers and resolves the runtime class without ensuring class initialization.
    pub fn_get_class: u32,
    pub ptr_methods: u32,
    pub ptr_fields: u32,
    /// Runtime-owned backing storage for the `java/lang/Class` instance fields and class static fields.
    pub ptr_class_fields: u32,
    pub unk14: u32,
    /// Number of 32-bit static field words allocated after the class instance fields.
    pub static_field_word_count: u32,
}

/// Count-prefixed class-name pointers used by class interface lists.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClassNames {
    pub count: u32,
    pub names: [u32; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClassField {
    pub ptr_class: u32,
    pub ptr_name: u32,
    pub ptr_descriptor: u32,
    pub flags: u16,
    pub unk2: u16,
    pub word_index: u32,
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
    pub ptr_descriptor: u32,
    pub access_flags: u16,
    /// Java argument words, including `this` for an instance method.
    pub argument_word_count: u16,
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
/// Member ranges used when linking an imported or exported class.
pub struct LgtJavaClassLink {
    pub ptr_name: u32,
    pub instance_field_offset: u16,
    pub instance_field_count: u16,
    pub static_field_offset: u16,
    pub static_field_count: u16,
    pub virtual_method_offset: u16,
    pub virtual_method_count: u16,
    pub interface_method_offset: u16,
    pub interface_method_count: u16,
    pub non_virtual_method_offset: u16,
    pub non_virtual_method_count: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClassLinks {
    pub count: u32,
    pub classes: [LgtJavaClassLink; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClassInstance {
    pub ptr_dispatch_table: u32,
    pub unk1: u32,
    pub ptr_fields: u32,
}
