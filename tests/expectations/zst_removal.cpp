#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct Option;

template<typename T>
struct MyStruct {
  uint32_t int_field;
  T generic_field;
};

extern "C" {

void option_u32(Option<uint32_t> arg);

void option_unit(Option<void> arg);

MyStruct<void> my_test();

}  // extern "C"
