#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint32_t MAY_FAIL_ARG_ZERO_ERR = 99;

template<typename T = void, typename E = void>
struct Result;

extern "C" {

Result<uint32_t> may_fail(uint32_t arg);

}  // extern "C"
