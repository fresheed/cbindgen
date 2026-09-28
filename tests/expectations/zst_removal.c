#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_1Z Option_1Z;

typedef struct Option_u32 Option_u32;

typedef struct {
  uint32_t int_field;
} MyStruct_1Z;

void option_u32(Option_u32 arg);

void option_unit(Option_1Z arg);

MyStruct_1Z my_test(void);
