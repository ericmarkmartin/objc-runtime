#![allow(non_snake_case)]

use crate::runtime::{
    class::{Class as ObjcClass, Flags},
    context::{ClassKey, Context},
    ivar::Ivar,
};
use std::{
    cell::LazyCell,
    ffi::{c_char, CStr, CString},
    ptr::NonNull,
    sync::Mutex,
};

static mut CONTEXT: LazyCell<Mutex<Context>> = LazyCell::new(|| Mutex::new(Context::new()));

pub type Class = Option<NonNull<ObjcClass>>;

static EMPTY_STRING: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };

#[no_mangle]
pub extern "C" fn class_getName(cls: Class) -> *const c_char {
    match cls {
        None => EMPTY_STRING.as_ptr(),
        Some(cls) => CString::new(unsafe { cls.as_ref().name.as_str() })
            .expect("shouldn't have a null byte")
            .into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn class_getSuperClass(cls: Class) -> Class {
    let superclass = unsafe { cls?.as_ref() }.superclass?;
    NonNull::new(unsafe { &mut CONTEXT.lock().expect("poisoned mutex").classes[superclass] })
}

#[no_mangle]
pub extern "C" fn class_isMetaClass(cls: Class) -> bool {
    match cls {
        None => false,
        Some(cls) => unsafe { cls.as_ref() }.info.contains(Flags::META),
    }
}

#[no_mangle]
pub extern "C" fn class_getInstanceSize(cls: Class) -> libc::size_t {
    match cls {
        None => 0,
        Some(_cls) => unimplemented!("class_getInstanceSize"),
    }
}

#[no_mangle]
pub extern "C" fn class_getInstanceVariable(
    cls: Class,
    name: *const c_char,
) -> Option<NonNull<Ivar>> {
    let name = unsafe { CStr::from_ptr(name) }
        .to_owned()
        .into_string()
        .expect("invalid utf8");

    unsafe {
        cls?.as_mut().ivars.iter_mut().find_map(|ivar| {
            if ivar.name == name {
                NonNull::new(ivar)
            } else {
                None
            }
        })
    }
}

#[no_mangle]
pub extern "C" fn class_getClassVariable(cls: Class, name: *const c_char) -> Option<NonNull<Ivar>> {
    let name = unsafe { CStr::from_ptr(name) }
        .to_owned()
        .into_string()
        .expect("invalid utf8");

    let mut context = unsafe { CONTEXT.lock().expect("poisoned mutex") };
    context.classes[unsafe { cls?.as_ref() }.metaclass]
        .ivars
        .iter_mut()
        .find_map(|ivar| {
            if ivar.name == name {
                NonNull::new(ivar)
            } else {
                None
            }
        })
}

#[no_mangle]
pub extern "C" fn class_addIvar(
    cls: Class,
    name: *const c_char,
    size: libc::size_t,
    alignment: u8,
    types: *const c_char,
) -> bool {
    match cls {
        None => false,
        Some(mut cls) => {
            let name = unsafe { CStr::from_ptr(name) }
                .to_owned()
                .into_string()
                .expect("invalid utf8");
            let types = unsafe { CStr::from_ptr(types) }
                .to_owned()
                .into_string()
                .expect("invalid utf8");
            let ivar = Ivar::new(name, size, alignment, types);
            unsafe { cls.as_mut() }.add_ivar(ivar)
        }
    }
}
