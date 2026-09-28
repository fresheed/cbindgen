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

template<typename T, typename R>
struct S {
  T x;
  R y;
};

extern "C" {

void option_u32(Option<uint32_t> arg);

void option_unit(Option<void> arg);

MyStruct<void> my_test();

S<uint32_t, void> f();

}  // extern "C"
