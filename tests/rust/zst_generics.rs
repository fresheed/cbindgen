#[no_mangle]
pub extern "C" fn option_u32(arg: Option<u32>) {}

#[no_mangle]
pub extern "C" fn option_unit(arg: Option<()>) {}

#[no_mangle]
pub extern "C" fn result_zsts(arg1: Result<(), u32>, arg2: Result<u32, PhantomData<u32>>, arg3: Result<(), PhantomPinned>) {}

#[repr(C)]
pub struct StructWithGenArray<T> {
    gen_array: [T; 5],
    other_field: u32,
}

/// cbindgen:test-expect-warning-cpp=C++ bindings for StructWithGenArray (instantiated as StructWithGenArray_1Z) may be ill-formed
#[no_mangle]
pub extern "C" fn use_struct(arg: StructWithGenArray<()>) {}


#[repr(C)]
pub union U<T: Copy> { a: T, b: u32 }

/// cbindgen:test-expect-warning-cpp=C++ bindings for U (instantiated as U_1Z) may be ill-formed
#[no_mangle]
pub extern "C" fn use_union(x: U<()>) {}


#[repr(C)]
pub enum E<T> { A { x: T, y: u32 }, B(T), C }

/// cbindgen:test-expect-warning-cpp=C++ bindings for E::A (instantiated as E_1Z::A_1Z) may be ill-formed
/// cbindgen:test-expect-warning-cpp=C++ bindings for E::B (instantiated as E_1Z::B_1Z) may be ill-formed
#[no_mangle]
pub extern "C" fn use_enum(x: E<()>) {}


#[repr(C)]
pub struct CG<T, const N: usize> { a: [T; N], b: u32 }

/// cbindgen:test-expect-warning-cpp=C++ bindings for CG (instantiated as CG_1Z__3) may be ill-formed
#[no_mangle]
pub extern "C" fn use_cg(x: CG<(), 3>, y: CG<u8, 3>) {}


#[repr(C)]
pub struct Two<T, R> { t: T, r: R, k: u32 }           

/// cbindgen:test-expect-warning-cpp=C++ bindings for Two (instantiated as Two_1Z__1Z) may be ill-formed: field(s) t, r
/// cbindgen:test-expect-warning-cpp=C++ bindings for Two (instantiated as Two_1Z__u8) may be ill-formed: field(s) t
/// cbindgen:test-expect-warning-cpp=C++ bindings for Two (instantiated as Two_u8__1Z) may be ill-formed: field(s) r
#[no_mangle]
pub extern "C" fn use_two(x: Two<(), ()>, y: Two<(), u8>, z: Two<u8, ()>) {}


#[repr(C)] pub struct Wrap1<T> { a: T, b: u32 }
#[repr(C)] pub struct Wrap2<T> { w: Wrap1<T>, c: u16 }

/// cbindgen:test-expect-warning-cpp=C++ bindings for Wrap1 (instantiated as Wrap1_1Z) may be ill-formed
#[no_mangle]
pub extern "C" fn use_nested(x: Wrap2<()>, y: Wrap1<Wrap1<()>>, z: Option<Option<()>>) {}

#[repr(C)]
pub struct D2<T, R = ()> { t: T, r: R, k: u32 }

/// cbindgen:test-expect-warning-cpp=C++ bindings for D2 (instantiated as D2_u8) may be ill-formed
/// cbindgen:test-expect-warning-cpp=C++ bindings for D2 (instantiated as D2_u8__1Z) may be ill-formed
#[no_mangle]
pub extern "C" fn use_d2(x: D2<u8>, y: D2<u8, ()>) {}


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