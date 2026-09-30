#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct Foo {
  uint32_t x;
};

extern "C" {

Foo<void> bar();

}  // extern "C"
