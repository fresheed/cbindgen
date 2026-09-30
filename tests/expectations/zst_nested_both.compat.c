#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque_1Z Opaque_1Z;

typedef struct Opaque_______c_void Opaque_______c_void;

typedef enum MyOption_______c_void_Tag {
  Some_______c_void,
  None_______c_void,
} MyOption_______c_void_Tag;

typedef struct MyOption_______c_void {
  MyOption_______c_void_Tag tag;
  union {
    struct {
      void (*some)(void);
    };
  };
} MyOption_______c_void;

typedef struct MyWrapper_______c_void {
  void (*inner)(void);
} MyWrapper_______c_void;

typedef struct S_1Z {
  void (*args_erased)(void);
  void (*args_erased_with_npo)(void);
  struct MyOption_______c_void args_erased_with_custom_enum;
  struct MyWrapper_______c_void args_erased_with_custom_struct;
  const struct Opaque_______c_void *args_erased_with_opaque;
  void (**args_erased_behind_ptr)(void);
  void (*args_erased_in_array[2])(void);
  void (*(*args_erased_in_ret)(void))(void);
  const void *arr_erased_behind_ptr;
  const struct Opaque_1Z *arr_erased_in_opaque;
  void (*args_always_erased)(void);
  const void *zst_behind_ptr;
  const struct Opaque_1Z *zst_in_opaque;
  void (*f_returns_zst)(uint32_t arg);
} S_1Z;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void use_S(struct S_1Z arg);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
