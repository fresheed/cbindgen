pub const MAY_FAIL_ARG_ZERO_ERR: u32 = 99;

#[no_mangle]
extern "C" fn may_fail(arg: u32) -> Result<(), NonZeroU32> {
    match arg == 0 {
        true => Err(NonZeroU32::new(MAY_FAIL_ARG_ZERO_ERR).unwrap()),
        false => Ok(()),
    }
}