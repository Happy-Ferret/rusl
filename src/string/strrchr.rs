use super::memrchr::memrchr;
use super::strlen::strlen;
use c_types::{c_int, c_schar, c_void};

#[no_mangle]
pub unsafe extern "C" fn strrchr(s: *const c_schar, c: c_int) -> *const c_schar {
    memrchr(s as *const c_void, c, strlen(s) + 1) as *const c_schar
}
