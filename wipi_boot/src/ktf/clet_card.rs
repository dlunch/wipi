use core::{ptr, slice};

use wipi_types::ktf::java::{JavaClass, JavaClassDescriptor, JavaMethodArray};

use crate::{
    handle_clet_event,
    ktf::java::{java_invoke_special, java_native_method_definition},
    paint_clet,
};

pub static mut CLET_CARD_CLASS: JavaClass = JavaClass {
    ptr_next: unsafe { &raw const CLET_CARD_CLASS.unk1 },
    unk1: 0,
    ptr_descriptor: &raw mut CLET_CARD_CLASS_DESCRIPTOR,
    ptr_vtable: 0,
    vtable_count: 0,
    unk_flag: 0,
};

static mut CLET_CARD_CLASS_DESCRIPTOR: JavaClassDescriptor = JavaClassDescriptor {
    ptr_name: c"CletCard".as_ptr(),
    unk1: 0,
    ptr_parent_class: c"org/kwis/msp/lcdui/Card".as_ptr() as _,
    ptr_methods: &CLET_CARD_CLASS_METHODS as *const _ as _,
    ptr_interfaces: 0,
    ptr_fields_or_element_type: 0,
    method_count: CLET_CARD_CLASS_METHODS.0.len() as u16 - 1,
    fields_size: 0,
    access_flag: 0,
    unk6: 0,
    unk7: 0,
    unk8: 0,
};

static CLET_CARD_CLASS_METHODS: JavaMethodArray<4> = JavaMethodArray([
    &java_native_method_definition(clet_card_init, &raw const CLET_CARD_CLASS, c".()V+<init>"),
    &java_native_method_definition(
        clet_card_paint,
        &raw const CLET_CARD_CLASS,
        c".(Lorg/kwis/msp/lcdui/Graphics;)V+paint",
    ),
    &java_native_method_definition(
        clet_card_key_notify,
        &raw const CLET_CARD_CLASS,
        c".(II)Z+keyNotify",
    ),
    ptr::null(),
]);

extern "C" fn clet_card_init(_: u32, args: *const *const ()) -> *const () {
    let args = unsafe { slice::from_raw_parts(args, 1) };
    let this = args[0];

    java_invoke_special(c"org/kwis/msp/lcdui/Card", c".()V+<init>", this, &[])
}

extern "C" fn clet_card_paint(_: u32, _args: *const *const ()) -> *const () {
    unsafe { paint_clet() }

    ptr::null()
}

extern "C" fn clet_card_key_notify(_: u32, args: *const *const ()) -> *const () {
    let args = unsafe { slice::from_raw_parts(args, 3) };
    let _this = args[0];
    let r#type = args[1] as i32;
    let key_code = args[2] as i32;

    unsafe { handle_clet_event(r#type, key_code, 0) };

    ptr::null() // TODO: return false
}
