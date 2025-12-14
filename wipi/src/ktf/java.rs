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

pub fn java_invoke_special(
    class_name: &CStr,
    fullname: &CStr,
    instance: *const (),
    args: &[*const ()],
) -> *const () {
    java_invoke(class_name, fullname, Some(instance), args)
}

pub fn java_invoke_static(class_name: &CStr, fullname: &CStr, args: &[*const ()]) -> *const () {
    java_invoke(class_name, fullname, None, args)
}

pub fn java_invoke_virtual(
    class_name: &CStr,
    fullname: &CStr,
    instance: *const (),
    args: &[*const ()],
) -> *const () {
    java_invoke(class_name, fullname, Some(instance), args)
}

pub fn java_instantiate(
    class_name: &CStr,
    constructor_type: &CStr,
    args: &[*const ()],
) -> *const () {
    unsafe {
        let class = get_java_class(class_name);

        let instance = ((*INIT_PARAM_4).fn_java_new)(class);

        // call ctor
        java_invoke_virtual(class_name, constructor_type, instance, args);

        instance
    }
}

fn get_java_class(class_name: &CStr) -> *const JavaClass {
    // TODO cache
    unsafe {
        let mut class_data = MaybeUninit::<*const JavaClass>::uninit();
        let result =
            ((*INIT_PARAM_4).fn_java_class_load)(class_data.as_mut_ptr(), class_name.as_ptr());

        if result != 0 {
            // TODO error handling
            panic!("Can't load class");
        }

        class_data.assume_init()
    }
}

fn java_invoke(
    class_name: &CStr,
    fullname: &CStr,
    instance: Option<*const ()>,
    args: &[*const ()],
) -> *const () {
    unsafe {
        let class = get_java_class(class_name);

        let method = ((*WIPI_JBINTERFACE).fn_get_java_method)(class, fullname.as_ptr());

        if method.is_null() {
            // TODO error handling
            panic!("Can't find method");
        }

        if (*method).access_flags & 0x100 == 0x100 {
            // native method
            let body: extern "C" fn(*const (), *const ()) -> *const () =
                transmute((*method).fn_body_native_or_exception_table);

            if let Some(instance) = instance {
                if args.len() == 0 {
                    (body)(ptr::null(), [instance].as_ptr() as *const ())
                } else if args.len() == 1 {
                    (body)(ptr::null(), [instance, args[0]].as_ptr() as *const ())
                } else if args.len() == 2 {
                    (body)(
                        ptr::null(),
                        [instance, args[0], args[1]].as_ptr() as *const (),
                    )
                } else {
                    panic!("Too many arguments");
                }
            } else {
                if args.len() == 0 {
                    (body)(ptr::null(), [].as_ptr() as *const ())
                } else if args.len() == 1 {
                    (body)(ptr::null(), [args[0]].as_ptr() as *const ())
                } else if args.len() == 2 {
                    (body)(ptr::null(), [args[0], args[1]].as_ptr() as *const ())
                } else {
                    panic!("Too many arguments");
                }
            }
        } else {
            let body: extern "C" fn(...) -> *const () = transmute((*method).fn_body);

            let unk: *const () = ptr::null();

            // TODO is it best?
            if instance.is_some() {
                if args.len() == 0 {
                    (body)(unk, instance.unwrap())
                } else if args.len() == 1 {
                    (body)(unk, instance.unwrap(), args[0])
                } else if args.len() == 2 {
                    (body)(unk, instance.unwrap(), args[0], args[1])
                } else if args.len() == 3 {
                    (body)(unk, instance.unwrap(), args[0], args[1], args[2])
                } else {
                    panic!("Too many arguments");
                }
            } else {
                if args.len() == 0 {
                    (body)(unk)
                } else if args.len() == 1 {
                    (body)(unk, args[0])
                } else if args.len() == 2 {
                    (body)(unk, args[0], args[1])
                } else if args.len() == 3 {
                    (body)(unk, args[0], args[1], args[2])
                } else {
                    panic!("Too many arguments");
                }
            }
        }
    }
}
