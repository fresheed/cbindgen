#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Option_1Z;

struct Option_u32;

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

struct MyStruct_1Z my_test(void);

struct S_u32__1Z f(void);

struct Either_1Z__u8 returns_either(void);

void CResultNoneAPIError_free(CResultNoneAPIError _res);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
