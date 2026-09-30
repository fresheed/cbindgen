#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct Option;

template<typename T = void, typename E = void>
struct Result;

template<typename T>
struct StructWithGenArray {
  T gen_array[5];
  uint32_t other_field;
};

template<typename T>
union U {
  T a;
  uint32_t b;
};

template<typename T>
struct E {
  enum class Tag {
    A,
    B,
    C,
  };

  struct A_Body {
    T x;
    uint32_t y;
  };

  struct B_Body {
    T _0;
  };

  Tag tag;
  union {
    A_Body a;
    B_Body b;
  };
};

template<typename T, uintptr_t N>
struct CG {
  T a[N];
  uint32_t b;
};

template<typename T, typename R>
struct Two {
  T t;
  R r;
  uint32_t k;
};

template<typename T>
struct Wrap1 {
  T a;
  uint32_t b;
};

template<typename T>
struct Wrap2 {
  Wrap1<T> w;
  uint16_t c;
};

template<typename T, typename R = void>
struct D2 {
  T t;
  R r;
  uint32_t k;
};

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

template<typename A, typename B>
struct Either {
  enum class Tag {
    Left,
    Right,
  };

  struct Left_Body {
    A _0;
  };

  struct Right_Body {
    B _0;
  };

  Tag tag;
  union {
    Left_Body left;
    Right_Body right;
  };
};

struct APIError {
  uint32_t err;
};

template<typename O, typename E>
struct CResultTempl {
  bool result_good;
  const O *result;
  const E *err;
};

using CResultNoneAPIError = CResultTempl<void, APIError>;

extern "C" {

void option_u32(Option<uint32_t> arg);

void option_unit(Option<void> arg);

void result_zsts(Result<void, uint32_t> arg1, Result<uint32_t, void> arg2, Result<void, void> arg3);

void use_struct(StructWithGenArray<void> arg);

void use_union(U<void> x);

void use_enum(E<void> x);

void use_cg(CG<void, 3> x, CG<uint8_t, 3> y);

void use_two(Two<void, void> x, Two<void, uint8_t> y, Two<uint8_t, void> z);

void use_nested(Wrap2<void> x, Wrap1<Wrap1<void>> y, Option<Option<void>> z);

void use_d2(D2<uint8_t> x, D2<uint8_t, void> y);

MyStruct<void> my_test();

S<uint32_t, void> f();

Either<void, uint8_t> returns_either();

void CResultNoneAPIError_free(CResultNoneAPIError _res);

}  // extern "C"
