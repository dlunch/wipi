use wipi_types::ktf::java::{
    JavaClass, JavaClassDescriptor, JavaMethodArray, JavaMethodDefinition,
};

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

static CLET_CLASS_METHODS: JavaMethodArray<2> =
    JavaMethodArray([&CLET_INIT_METHOD, core::ptr::null()]);

static CLET_INIT_METHOD: JavaMethodDefinition = JavaMethodDefinition {
    fn_body: 0,
    ptr_class: &CLET_CLASS,
    fn_body_native_or_exception_table: clet_init,
    ptr_name: c".()V+<init>".as_ptr(),
    exception_table_count: 0,
    unk3: 0,
    index_in_vtable: 0,
    access_flags: 0x100, // ACC_NATIVE // TODO use enum
    unk6: 0,
};

extern "C" fn clet_init(_args: u32) -> u32 {
    // TODO call superclass <init>
    0
}
