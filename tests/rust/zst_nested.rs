#[repr(C)]
pub enum MyOption<T> {
    Some(T),
    None,
}

#[repr(C)]
pub struct MyWrapper<T> {
    inner: T,
}

struct Opaque<T> {
    inner: T,
}

#[repr(C)]
pub struct S<T> {
    args_erased: extern "C" fn(arg: T),

    args_erased_with_npo: Option<extern "C" fn(arg: T)>,
    args_erased_with_custom_enum: MyOption<extern "C" fn(arg: T)>,
    args_erased_with_custom_struct: MyWrapper<extern "C" fn(arg: T)>,
    args_erased_with_opaque: *const Opaque<extern "C" fn(arg: T)>,

    args_erased_behind_ptr: *const extern "C" fn(arg: T),
    args_erased_in_array: [extern "C" fn(arg: T); 2],
    args_erased_in_ret: extern "C" fn() -> extern "C" fn(arg: T),
    
    arr_erased_behind_ptr: *const [T; 2],
    arr_erased_in_opaque: *const Opaque<[T; 2]>,
        
    // no warnings
    args_always_erased: extern "C" fn(arg: ()), // does not depent on T
    zst_behind_ptr: *const T, // *void is valid
    zst_in_opaque: *const Opaque<T>, // *Opaque<void> is valid
    f_returns_zst: extern "C" fn(arg: u32) -> T, // void return is valid
}

/// cbindgen:test-expect-warning-cpp=C++ bindings for S (instantiated as S_1Z) may be ill-formed
/// cbindgen:test-expect-warning-cpp=field(s) args_erased, args_erased_with_NPO, args_erased_with_custom_enum, args_erased_with_custom_struct, args_erased_with_opaque, args_erased_behind_ptr, args_erased_in_array, args_erased_in_ret, arr_erased_behind_ptr, arr_erased_in_opaque are
#[no_mangle]
pub extern "C" fn use_S(arg: S<()>) {}    