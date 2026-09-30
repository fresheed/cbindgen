//! cbindgen:test-expect-no-warnings
// A simpler case of #659 (by stevenengler). 
// _phantom field of Foo is zero-sized regardless of T, so can always be removed

#[repr(C)]
struct Foo<T> {
    x: u32,    
    _phantom: std::marker::PhantomData<T>,
}

#[no_mangle]
pub extern "C" fn bar() -> Foo<()> {
    todo!()
}