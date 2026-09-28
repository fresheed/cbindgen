#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_1Z Option_1Z;

typedef struct Option_u32 Option_u32;

typedef struct {
  uint32_t int_field;
} MyStruct_1Z;

typedef struct {
  uint32_t x;
} S_u32__1Z;

typedef enum {
  Left_1Z__u8,
  Right_1Z__u8,
} Either_1Z__u8_Tag;

typedef struct {
  Either_1Z__u8_Tag tag;
  union {
    struct {

    };
    struct {
      uint8_t right;
    };
  };
} Either_1Z__u8;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void option_u32(Option_u32 arg);

void option_unit(Option_1Z arg);

MyStruct_1Z my_test(void);

S_u32__1Z f(void);

Either_1Z__u8 returns_either(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
