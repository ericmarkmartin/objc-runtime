use super::global_context::CONTEXT;
use crate::runtime::selector::SEL;
use std::ffi::{c_char, CStr};
use std::ptr::NonNull;

#[no_mangle]
pub extern "C" fn sel_getName(sel: SEL) -> *const c_char {
    match sel {
        None => std::ptr::null(),
        Some(sel) => unsafe { sel.as_ref() }.selector_info.name.as_ptr(),
    }
}

#[no_mangle]
pub extern "C" fn sel_isEqual(lhs: SEL, rhs: SEL) -> bool {
    lhs == rhs
}

#[no_mangle]
pub unsafe extern "C" fn sel_registerName(name: *const c_char) -> SEL {
    let name = unsafe { CStr::from_ptr(name) }.to_owned();
    let mut context = CONTEXT.write().expect("poisoned rwlock");
    let selector_key = context.allocate_selector(name);
    NonNull::new(&mut context.selectors[selector_key] as *mut _)
}

pub unsafe extern "C" fn sel_getUid(name: *const c_char) -> SEL {
    sel_registerName(name)
}
