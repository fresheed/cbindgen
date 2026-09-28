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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void option_u32(struct Option_u32 arg);

void option_unit(struct Option_1Z arg);

struct MyStruct_1Z my_test(void);

struct S_u32__1Z f(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
