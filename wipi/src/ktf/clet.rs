use core::{ptr, slice};

use wipi_types::ktf::java::{JavaClass, JavaClassDescriptor, JavaMethodArray};

use crate::{
    ktf::java::{
        java_instantiate, java_invoke_special, java_invoke_static, java_invoke_virtual,
        java_native_method_definition,
    },
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
    method_count: CLET_CLASS_METHODS.0.len() as u16 - 1,
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
    ptr::null(),
]);

extern "C" fn clet_init(_: u32, args: *const *const ()) -> *const () {
    let args = unsafe { slice::from_raw_parts(args, 1) };
    let this = args[0];

    java_invoke_special(c"org/kwis/msp/lcdui/Jlet", c".()V+<init>", &[this])
}

extern "C" fn clet_start_app(_: u32, _args: *const *const ()) -> *const () {
    let display = java_invoke_static(
        c"org/kwis/msp/lcdui/Display",
        c".(Ljava/lang/String;)Lorg/kwis/msp/lcdui/Display;+getDisplay",
        &[],
    );

    let clet_card = java_instantiate(c"CletCard", c".()V+<init>", &[]);

    java_invoke_virtual(
        c"org/kwis/msp/lcdui/Display",
        c".(Lorg/kwis/msp/lcdui/Card;)V+pushCard",
        &[display, clet_card],
    );

    unsafe {
        start_clet();
    }

    ptr::null()
}
