#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_1Z Option_1Z;

typedef struct Option_u32 Option_u32;

typedef struct MyStruct_1Z {
  uint32_t int_field;
} MyStruct_1Z;

typedef struct S_u32__1Z {
  uint32_t x;
} S_u32__1Z;

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
