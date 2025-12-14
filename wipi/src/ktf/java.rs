use core::{
    ffi::CStr,
    mem::{MaybeUninit, transmute},
    ptr,
};

use wipi_types::ktf::java::{JavaClass, JavaMethodDefinition, JavaNativeMethodBody};

use super::globals::{INIT_PARAM_4, WIPI_JBINTERFACE};

pub const fn java_native_method_definition(
    body: JavaNativeMethodBody,
    class: *const JavaClass,
    fullname: &CStr,
) -> JavaMethodDefinition {
    JavaMethodDefinition {
        fn_body: ptr::null(),
        ptr_class: class,
        fn_body_native_or_exception_table: body,
        ptr_name: fullname.as_ptr(),
        exception_table_count: 0,
        unk3: 0,
        index_in_vtable: 0,
        access_flags: 0x100, // ACC_NATIVE // TODO use enum
        unk6: 0,
    }
}

pub fn java_invoke_special(class: &CStr, fullname: &CStr, args: &[*const ()]) -> *const () {
    java_invoke(class, fullname, args)
}

pub fn java_invoke_static(class: &CStr, fullname: &CStr, args: &[*const ()]) -> *const () {
    java_invoke(class, fullname, args)
}

pub fn java_invoke_virtual(class: &CStr, fullname: &CStr, args: &[*const ()]) -> *const () {
    java_invoke(class, fullname, args)
}

pub fn java_instantiate(_class: &CStr, _constructor: &CStr, _args: &[*const ()]) -> *const () {
    ptr::null() // TODO
}

fn java_invoke(class: &CStr, fullname: &CStr, args: &[*const ()]) -> *const () {
    // TODO cache
    unsafe {
        let mut class_data = MaybeUninit::<*const JavaClass>::uninit();
        let result = ((*INIT_PARAM_4).fn_java_class_load)(class_data.as_mut_ptr(), class.as_ptr());

        if result != 0 {
            // TODO error handling
            panic!("Can't load class");
        }

        let class_data = class_data.assume_init();

        let method = ((*WIPI_JBINTERFACE).fn_get_java_method)(class_data, fullname.as_ptr());

        if method.is_null() {
            // TODO error handling
            panic!("Can't find method");
        }

        let body: extern "C" fn(*const (), ...) -> *const () = transmute((*method).fn_body);

        (body)(ptr::null(), args[0]) // TODO hardcoded argument
    }
}
