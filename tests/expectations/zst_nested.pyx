from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Opaque_1Z:
    pass

  ctypedef struct Opaque_______c_void:
    pass

  ctypedef enum MyOption_______c_void_Tag:
    Some_______c_void,
    None_______c_void,

  ctypedef struct MyOption_______c_void:
    MyOption_______c_void_Tag tag;
    void (*some)();

  ctypedef struct MyWrapper_______c_void:
    void (*inner)();

  ctypedef struct S_1Z:
    void (*args_erased)();
    void (*args_erased_with_npo)();
    MyOption_______c_void args_erased_with_custom_enum;
    MyWrapper_______c_void args_erased_with_custom_struct;
    const Opaque_______c_void *args_erased_with_opaque;
    void (**args_erased_behind_ptr)();
    void (*args_erased_in_array[2])();
    void (*(*args_erased_in_ret)())();
    const void *arr_erased_behind_ptr;
    const Opaque_1Z *arr_erased_in_opaque;
    void (*args_always_erased)();
    const void *zst_behind_ptr;
    const Opaque_1Z *zst_in_opaque;
    void (*f_returns_zst)(uint32_t arg);

  void use_S(S_1Z arg);
