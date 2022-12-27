use std::ffi::CStr;

pub(crate) static EMPTY_STRING: &'static CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };
