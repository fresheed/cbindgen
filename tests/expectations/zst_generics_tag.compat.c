#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Option_1Z;

struct Option_Option_1Z;

struct Option_u32;

struct Result_1Z__1Z;

struct Result_1Z__u32;

struct Result_u32__1Z;

struct StructWithGenArray_1Z {
  uint32_t other_field;
};

union U_1Z {
  uint32_t b;
};

enum E_1Z_Tag {
  A_1Z,
  B_1Z,
  C_1Z,
};

struct A_Body_1Z {
  uint32_t y;
};

struct E_1Z {
  enum E_1Z_Tag tag;
  union {
    struct A_Body_1Z a;
    struct {

    };
  };
};

struct CG_1Z__3 {
  uint32_t b;
};

struct CG_u8__3 {
  uint8_t a[3];
  uint32_t b;
};

struct Two_1Z__1Z {
  uint32_t k;
};

struct Two_1Z__u8 {
  uint8_t r;
  uint32_t k;
};

struct Two_u8__1Z {
  uint8_t t;
  uint32_t k;
};

struct Wrap1_1Z {
  uint32_t b;
};

struct Wrap2_1Z {
  struct Wrap1_1Z w;
  uint16_t c;
};

struct Wrap1_Wrap1_1Z {
  struct Wrap1_1Z a;
  uint32_t b;
};

struct D2_u8 {
  uint8_t t;
  uint32_t k;
};

struct D2_u8__1Z {
  uint8_t t;
  uint32_t k;
};

struct MyStruct_1Z {
  uint32_t int_field;
};

struct S_u32__1Z {
  uint32_t x;
};

enum Either_1Z__u8_Tag {
  Left_1Z__u8,
  Right_1Z__u8,
};

struct Either_1Z__u8 {
  enum Either_1Z__u8_Tag tag;
  union {
    struct {

    };
    struct {
      uint8_t right;
    };
  };
};

struct APIError {
  uint32_t err;
};

struct CResultTempl_1Z__APIError {
  bool result_good;
  const void *result;
  const struct APIError *err;
};

typedef struct CResultTempl_1Z__APIError CResultNoneAPIError;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void option_u32(struct Option_u32 arg);

void option_unit(struct Option_1Z arg);

void result_zsts(struct Result_1Z__u32 arg1, struct Result_u32__1Z arg2, struct Result_1Z__1Z arg3);

void use_struct(struct StructWithGenArray_1Z arg);

void use_union(union U_1Z x);

void use_enum(struct E_1Z x);

void use_cg(struct CG_1Z__3 x, struct CG_u8__3 y);

void use_two(struct Two_1Z__1Z x, struct Two_1Z__u8 y, struct Two_u8__1Z z);

void use_nested(struct Wrap2_1Z x, struct Wrap1_Wrap1_1Z y, struct Option_Option_1Z z);

void use_d2(struct D2_u8 x, struct D2_u8__1Z y);

struct MyStruct_1Z my_test(void);

struct S_u32__1Z f(void);

struct Either_1Z__u8 returns_either(void);

void CResultNoneAPIError_free(CResultNoneAPIError _res);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
