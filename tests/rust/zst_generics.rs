#[no_mangle]
pub extern "C" fn option_u32(arg: Option<u32>) {}

#[no_mangle]
pub extern "C" fn option_unit(arg: Option<()>) {}

#[no_mangle]
pub extern "C" fn result_zsts(arg1: Result<(), u32>, arg2: Result<u32, PhantomData<u32>>, arg3: Result<(), PhantomData<u32>>) {}

#[repr(C)]
pub struct StructWithGenArray<T> {
    gen_array: [T; 5],
    other_field: u32,
}

/// cbindgen:test-expect-warning-cpp=C++ bindings for StructWithGenArray (instantiated as StructWithGenArray_1Z) may be ill-formed
#[no_mangle]
pub extern "C" fn use_struct(arg: StructWithGenArray<()>) {}

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


// --------------------------
// #527, example by TheBlueMatt (simplified): pointers to ZST can be safely represented as *void
#[repr(C)]
pub struct CResultTempl<O, E> {
	pub result_good: bool,
	pub result: *const O,
	pub err: *const E,
}

// simplified the rest of example by removing unknown types and replacing static with function
#[repr(C)]
pub struct APIError {
    err: u32,
}

#[no_mangle]
pub type CResultNoneAPIError = CResultTempl<(), APIError>;

#[no_mangle]
pub extern "C" fn CResultNoneAPIError_free(_res: CResultNoneAPIError) {}
// --------------------------