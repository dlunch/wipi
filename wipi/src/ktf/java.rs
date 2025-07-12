use core::ffi::c_char;

use wipi_types::ktf::java::{JavaClass, JavaMethodDefinition};

pub const fn java_native_method_definition(
    body: extern "C" fn(u32) -> u32,
    class: *const JavaClass,
    name: *const c_char,
) -> JavaMethodDefinition {
    JavaMethodDefinition {
        fn_body: 0,
        ptr_class: class,
        fn_body_native_or_exception_table: body,
        ptr_name: name,
        exception_table_count: 0,
        unk3: 0,
        index_in_vtable: 0,
        access_flags: 0x100, // ACC_NATIVE // TODO use enum
        unk6: 0,
    }
}
