use wipi_types::ktf::java::{JavaClass, JavaClassDescriptor, JavaMethodArray};

use crate::ktf::java::java_native_method_definition;

#[allow(dead_code)]
unsafe extern "C" {
    #[link_name = "startClet"]
    fn start_clet();
    #[link_name = "destroyClet"]
    fn destroy_clet();
    #[link_name = "paintClet"]
    fn paint_clet();
    #[link_name = "pauseClet"]
    fn pause_clet();
    #[link_name = "resumeClet"]
    fn resume_clet();

}

pub static CLET_CLASS: JavaClass = JavaClass {
    ptr_next: &CLET_CLASS.unk1,
    unk1: 0,
    ptr_descriptor: &CLET_CLASS_DESCRIPTOR,
    ptr_vtable: 0,
    vtable_count: 0,
    unk_flag: 0,
};

static CLET_CLASS_DESCRIPTOR: JavaClassDescriptor = JavaClassDescriptor {
    ptr_name: c"Clet".as_ptr(),
    unk1: 0,
    ptr_parent_class: 0,
    ptr_methods: &CLET_CLASS_METHODS as *const _ as _,
    ptr_interfaces: 0,
    ptr_fields_or_element_type: 0,
    method_count: 1,
    fields_size: 0,
    access_flag: 0,
    unk6: 0,
    unk7: 0,
    unk8: 0,
};

static CLET_CLASS_METHODS: JavaMethodArray<2> = JavaMethodArray([
    &java_native_method_definition(clet_init, &CLET_CLASS, c".()V+<init>".as_ptr()),
    core::ptr::null(),
]);

extern "C" fn clet_init(_args: u32) -> u32 {
    // TODO call superclass <init>
    0
}
