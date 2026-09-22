#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define MAY_FAIL_ARG_ZERO_ERR 99

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

uint32_t may_fail(uint32_t arg);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
