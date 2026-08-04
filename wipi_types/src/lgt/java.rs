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
    pub access_flags: u32,
    /// Next class record in the compiler-generated singly linked list.
    pub ptr_next_class: u32,
    pub ptr_name: u32,
    /// Points to the class-pointer word inside the instance field initializer record.
    pub ptr_instance_field_initializer_class: u32,
    pub ptr_super_class_name: u32,
    pub unk4: u32,
    /// Total 32-bit instance field words, including words inherited from the superclass.
    pub instance_field_word_count: u16,
    /// Registration state checked before resolving the runtime class.
    pub link_state: u16,
    pub unk7: u32,
    /// Describes a callback that initializes object fields, not `<init>` or `<clinit>`.
    pub ptr_instance_field_initializer_record: u32,
    pub unk9: u8,
    pub unk10: u8,
    pub unk11: u16,
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
    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
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
pub struct LgtJavaClassInstance {
    pub ptr_dispatch_table: u32,
    pub unk1: u32,
    pub ptr_fields: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LgtJavaClassInstanceFields {
    pub unk1: [u32; 4],
    pub unk2: u16,
    pub unk3: u16,
    pub fields: [u32; 0],
}
