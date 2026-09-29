#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque_1Z Opaque_1Z;

typedef struct Opaque_______c_void Opaque_______c_void;

typedef enum {
  Some_______c_void,
  None_______c_void,
} MyOption_______c_void_Tag;

typedef struct {
  MyOption_______c_void_Tag tag;
  union {
    struct {
      void (*some)(void);
    };
  };
} MyOption_______c_void;

typedef struct {
  void (*inner)(void);
} MyWrapper_______c_void;

typedef struct {
  void (*args_erased)(void);
  void (*args_erased_with_NPO)(void);
  MyOption_______c_void args_erased_with_custom_enum;
  MyWrapper_______c_void args_erased_with_custom_struct;
  const Opaque_______c_void *args_erased_with_opaque;
  void (**args_erased_behind_ptr)(void);
  void (*args_erased_in_array[2])(void);
  void (*(*args_erased_in_ret)(void))(void);
  const void *arr_erased_behind_ptr;
  const Opaque_1Z *arr_erased_in_opaque;
  void (*args_always_erased)(void);
  const void *zst_behind_ptr;
  const Opaque_1Z *zst_in_opaque;
  void (*f_returns_zst)(uint32_t arg);
} S_1Z;

void use_S(S_1Z arg);
