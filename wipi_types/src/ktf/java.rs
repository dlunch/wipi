use bytemuck::{Pod, Zeroable};

cfg_if::cfg_if! {
    if #[cfg(target_os = "none")] {
        use core::ffi::c_char;

        type JavaClassPtrNext = *const u32;
        type JavaClassDescriptorPtr = *const JavaClassDescriptor;
        type JavaClassDescriptorNamePtr = *const c_char;
        type JavaClassPtr = *const JavaClass;
        type StringPtr = *const c_char;
        type JavaMethodBody = unsafe extern "C" fn (u32) -> u32;
        type JavaMethodArrayPtr = *const JavaMethodDefinition;
    } else {
        type JavaClassPtrNext = u32;
        type JavaClassDescriptorPtr = u32;
        type JavaClassDescriptorNamePtr = u32;
        type JavaClassPtr = u32;
        type StringPtr = u32;
        type JavaMethodBody = u32;
        type JavaMethodArrayPtr = u32;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct JavaClass {
    pub ptr_next: JavaClassPtrNext,
    pub unk1: u32,
    pub ptr_descriptor: JavaClassDescriptorPtr,
    pub ptr_vtable: u32,
    pub vtable_count: u16,
    pub unk_flag: u16,
}

unsafe impl Sync for JavaClass {}
unsafe impl Send for JavaClass {}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct JavaClassDescriptor {
    pub ptr_name: JavaClassDescriptorNamePtr,
    pub unk1: u32,
    pub ptr_parent_class: u32,
    pub ptr_methods: JavaMethodArrayPtr,
    pub ptr_interfaces: u32,
    pub ptr_fields_or_element_type: u32, // for array class, this is element type
    pub method_count: u16,
    pub fields_size: u16,
    pub access_flag: u16,
    pub unk6: u16,
    pub unk7: u16,
    pub unk8: u16,
}

unsafe impl Sync for JavaClassDescriptor {}
unsafe impl Send for JavaClassDescriptor {}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct JavaClassInstance {
    pub ptr_fields: u32,
    pub ptr_class: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct JavaClassInstanceFields {
    pub vtable_index: u32, // left shifted by 5
    pub fields: [u32; 1],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct JavaFieldDefinition {
    pub access_flags: u32,
    pub ptr_class: u32,
    pub ptr_name: u32,
    pub offset_or_value: u32,
}

#[repr(transparent)]
pub struct JavaMethodArray<const N: usize>(pub [*const JavaMethodDefinition; N]);

unsafe impl<const N: usize> Sync for JavaMethodArray<N> {}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(not(target_os = "none"), derive(Pod, Zeroable))]
pub struct JavaMethodDefinition {
    pub fn_body: u32,
    pub ptr_class: JavaClassPtr,
    pub fn_body_native_or_exception_table: JavaMethodBody,
    pub ptr_name: StringPtr,
    pub exception_table_count: u16,
    pub unk3: u16,
    pub index_in_vtable: u16,
    pub access_flags: u16,
    pub unk6: u32,
}

unsafe impl Sync for JavaMethodDefinition {}
unsafe impl Send for JavaMethodDefinition {}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct JavaMethodExceptionTableEntry {
    pub from_pc: u32,
    pub to_pc: u32,
    pub target: u32,
    pub ptr_class: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct JavaExceptionHandler {
    pub ptr_method: u32,
    pub ptr_this: u32,
    pub ptr_old_handler: u32,
    pub current_pc: u32,
    pub unk3: u32,
    pub ptr_functions: u32, // function table to restore context
    pub context: [u32; 11], // r4-lr
}
