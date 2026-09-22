from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const uint32_t MAY_FAIL_ARG_ZERO_ERR # = 99

  cdef struct Result_u32:
    pass

  Result_u32 may_fail(uint32_t arg);
