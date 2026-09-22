#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define MAY_FAIL_ARG_ZERO_ERR 99

typedef struct Result_u32 Result_u32;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

Result_u32 may_fail(uint32_t arg);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
