#![allow(non_snake_case)]
use crate::runtime::{class::Class, ivar::Ivar};
use std::ffi::{c_char, CStr};

pub type BOOL = bool;

extern "C" fn class_addIvar(
    cls: *mut Class,
    name: *const c_char,
    size: usize,
    alignment: u8,
    types: *const c_char,
) -> bool {
    unsafe {
        // TODO: should we return [false] if we fail the utf8 validation?
        let name = CStr::from_ptr(name)
            .to_owned()
            .into_string()
            .expect("invalid utf8");
        let types = CStr::from_ptr(types)
            .to_owned()
            .into_string()
            .expect("invalid utf8");
        let ivar = Ivar::new(name, size, alignment, types);
        (*cls).add_ivar(ivar)
    }
}
