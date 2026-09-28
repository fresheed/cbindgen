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
// also, #99 has a very similar example
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


// --------------------------
// #527, example by GoldsteinE
// This ends up with an empty Left variant, which is not C standard, but allowed by GCC
#[repr(C)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

/// cbindgen:test-expect-warning-cpp=C++ bindings for Either::Left (instantiated as Either_1Z__u8::Left_1Z__u8) may be ill-formed
#[no_mangle]
extern "C" fn returns_either() -> Either<(), u8> {
    Either::Right(42)
}
// --------------------------