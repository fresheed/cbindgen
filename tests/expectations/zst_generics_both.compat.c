#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_1Z Option_1Z;

typedef struct Option_u32 Option_u32;

typedef struct Result_1Z__1Z Result_1Z__1Z;

typedef struct Result_1Z__u32 Result_1Z__u32;

typedef struct Result_u32__1Z Result_u32__1Z;

typedef struct StructWithGenArray_1Z {
  uint32_t other_field;
} StructWithGenArray_1Z;

typedef struct MyStruct_1Z {
  uint32_t int_field;
} MyStruct_1Z;

typedef struct S_u32__1Z {
  uint32_t x;
} S_u32__1Z;

typedef enum Either_1Z__u8_Tag {
  Left_1Z__u8,
  Right_1Z__u8,
} Either_1Z__u8_Tag;

typedef struct Either_1Z__u8 {
  Either_1Z__u8_Tag tag;
  union {
    struct {

    };
    struct {
      uint8_t right;
    };
  };
} Either_1Z__u8;

typedef struct APIError {
  uint32_t err;
} APIError;

typedef struct CResultTempl_1Z__APIError {
  bool result_good;
  const void *result;
  const struct APIError *err;
} CResultTempl_1Z__APIError;

typedef struct CResultTempl_1Z__APIError CResultNoneAPIError;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void option_u32(struct Option_u32 arg);

void option_unit(struct Option_1Z arg);

void result_zsts(struct Result_1Z__u32 arg1, struct Result_u32__1Z arg2, struct Result_1Z__1Z arg3);

void use_struct(struct StructWithGenArray_1Z arg);

struct MyStruct_1Z my_test(void);

struct S_u32__1Z f(void);

struct Either_1Z__u8 returns_either(void);

void CResultNoneAPIError_free(CResultNoneAPIError _res);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
