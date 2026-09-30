from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Option_1Z:
    pass

  ctypedef struct Option_Option_1Z:
    pass

  ctypedef struct Option_u32:
    pass

  ctypedef struct Result_1Z__1Z:
    pass

  ctypedef struct Result_1Z__u32:
    pass

  ctypedef struct Result_u32__1Z:
    pass

  ctypedef struct StructWithGenArray_1Z:
    uint32_t other_field;

  ctypedef union U_1Z:
    uint32_t b;

  ctypedef enum E_1Z_Tag:
    A_1Z,
    B_1Z,
    C_1Z,

  ctypedef struct A_Body_1Z:
    uint32_t y;

  ctypedef struct E_1Z:
    E_1Z_Tag tag;
    A_Body_1Z a;


  ctypedef struct CG_1Z__3:
    uint32_t b;

  ctypedef struct CG_u8__3:
    uint8_t a[3];
    uint32_t b;

  ctypedef struct Two_1Z__1Z:
    uint32_t k;

  ctypedef struct Two_1Z__u8:
    uint8_t r;
    uint32_t k;

  ctypedef struct Two_u8__1Z:
    uint8_t t;
    uint32_t k;

  ctypedef struct Wrap1_1Z:
    uint32_t b;

  ctypedef struct Wrap2_1Z:
    Wrap1_1Z w;
    uint16_t c;

  ctypedef struct Wrap1_Wrap1_1Z:
    Wrap1_1Z a;
    uint32_t b;

  ctypedef struct D2_u8:
    uint8_t t;
    uint32_t k;

  ctypedef struct D2_u8__1Z:
    uint8_t t;
    uint32_t k;

  ctypedef struct MyStruct_1Z:
    uint32_t int_field;

  ctypedef struct S_u32__1Z:
    uint32_t x;

  ctypedef enum Either_1Z__u8_Tag:
    Left_1Z__u8,
    Right_1Z__u8,

  ctypedef struct Either_1Z__u8:
    Either_1Z__u8_Tag tag;

    uint8_t right;

  ctypedef struct APIError:
    uint32_t err;

  ctypedef struct CResultTempl_1Z__APIError:
    bool result_good;
    const void *result;
    const APIError *err;

  ctypedef CResultTempl_1Z__APIError CResultNoneAPIError;

  void option_u32(Option_u32 arg);

  void option_unit(Option_1Z arg);

  void result_zsts(Result_1Z__u32 arg1, Result_u32__1Z arg2, Result_1Z__1Z arg3);

  void use_struct(StructWithGenArray_1Z arg);

  void use_union(U_1Z x);

  void use_enum(E_1Z x);

  void use_cg(CG_1Z__3 x, CG_u8__3 y);

  void use_two(Two_1Z__1Z x, Two_1Z__u8 y, Two_u8__1Z z);

  void use_nested(Wrap2_1Z x, Wrap1_Wrap1_1Z y, Option_Option_1Z z);

  void use_d2(D2_u8 x, D2_u8__1Z y);

  MyStruct_1Z my_test();

  S_u32__1Z f();

  Either_1Z__u8 returns_either();

  void CResultNoneAPIError_free(CResultNoneAPIError _res);
