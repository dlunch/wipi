use core::ptr::null;

use wipi_types::ktf::java::{JavaClass, JavaClassDescriptor, JavaMethodArray};

use crate::{
    ktf::java::{java_invoke_special, java_native_method_definition},
    start_clet,
};

pub static mut CLET_CLASS: JavaClass = JavaClass {
    ptr_next: unsafe { &raw const CLET_CLASS.unk1 },
    unk1: 0,
    ptr_descriptor: &raw mut CLET_CLASS_DESCRIPTOR,
    ptr_vtable: 0,
    vtable_count: 0,
    unk_flag: 0,
};

static mut CLET_CLASS_DESCRIPTOR: JavaClassDescriptor = JavaClassDescriptor {
    ptr_name: c"Clet".as_ptr(),
    unk1: 0,
    ptr_parent_class: c"org/kwis/msp/lcdui/Jlet".as_ptr() as _,
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

static CLET_CLASS_METHODS: JavaMethodArray<3> = JavaMethodArray([
    &java_native_method_definition(clet_init, &raw const CLET_CLASS, c".()V+<init>"),
    &java_native_method_definition(
        clet_start_app,
        &raw const CLET_CLASS,
        c".([Ljava/lang/String;)V+startApp",
    ),
    null(),
]);

extern "C" fn clet_init(_: u32, args: *const ()) -> u32 {
    let this = unsafe { *(args as *const u32) };

    java_invoke_special(c"org/kwis/msp/lcdui/Jlet", c".()V+<init>", &[this])
}

extern "C" fn clet_start_app(_: u32, _args: *const ()) -> u32 {
    unsafe {
        start_clet();
    }

    0
}
