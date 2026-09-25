#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct Option;

extern "C" {

void option_u32(Option<uint32_t> arg);

void option_unit(Option<> arg);

}  // extern "C"
