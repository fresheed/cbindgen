#[no_mangle]
pub extern "C" fn option_u32(arg: Option<u32>) {}

#[no_mangle]
pub extern "C" fn option_unit(arg: Option<()>) {}

// --------------------------
// #659
#[repr(C)]
pub struct MyStruct<T> { int_field: u32, generic_field: T }

#[no_mangle]
pub extern "C" fn my_test() -> MyStruct<()> { loop {} }
// --------------------------
