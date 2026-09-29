#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct Opaque;

template<typename T>
struct MyOption {
  enum class Tag {
    Some,
    None,
  };

  struct Some_Body {
    T _0;
  };

  Tag tag;
  union {
    Some_Body some;
  };
};

template<typename T>
struct MyWrapper {
  T inner;
};

template<typename T>
struct S {
  void (*args_erased)(T arg);
  void (*args_erased_with_NPO)(T arg);
  MyOption<void(*)(T arg)> args_erased_with_custom_enum;
  MyWrapper<void(*)(T arg)> args_erased_with_custom_struct;
  const Opaque<void(*)(T arg)> *args_erased_with_opaque;
  void (**args_erased_behind_ptr)(T arg);
  void (*args_erased_in_array[2])(T arg);
  void (*(*args_erased_in_ret)())(T arg);
  const T (*arr_erased_behind_ptr)[2];
  const Opaque<T[2]> *arr_erased_in_opaque;
  void (*args_always_erased)();
  const T *zst_behind_ptr;
  const Opaque<T> *zst_in_opaque;
  T (*f_returns_zst)(uint32_t arg);
};

extern "C" {

void use_S(S<void> arg);

}  // extern "C"
