#[no_mangle]
pub extern "C" fn option_u32(arg: Option<u32>) {}

#[no_mangle]
pub extern "C" fn option_unit(arg: Option<()>) {}

// --------------------------
// #659
#[repr(C)]
pub struct MyStruct<T> { int_field: u32, generic_field: T }

/// cbindgen:test-expect-warning-cpp=C++ bindings for MyStruct (instantiated as MyStruct_1Z) may be ill-formed
#[no_mangle]
pub extern "C" fn my_test() -> MyStruct<()> { loop {} }
// --------------------------


// --------------------------
// #228, original issue
#[repr(C)]
pub struct S<T, R> {
    x: T,
    y: R,
}

/// cbindgen:test-expect-warning-cpp=C++ bindings for S (instantiated as S_u32__1Z) may be ill-formed
#[no_mangle]
pub extern "C" fn f() -> S<u32, ()> {
    panic!()
}
// --------------------------