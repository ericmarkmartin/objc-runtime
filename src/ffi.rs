#![allow(non_snake_case)]

use crate::runtime::{
    class::{Class as ObjcClass, ClassKind},
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

// TODO: check repr here (classkey is )
type Class = Option<NonNull<ClassKey>>;
type ClassV2 = Option<NonNull<ObjcClass>>;

static EMPTY_STRING: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };

pub extern "C" fn class_getName(cls: Class) -> *const c_char {
    match cls {
        None => EMPTY_STRING.as_ptr(),
        Some(cls) => CString::new(unsafe {
            CONTEXT.lock().expect("poisoned mutex").classes[*cls.as_ref()]
                .name
                .as_str()
        })
        .expect("shouldn't have a null byte")
        // TODO: this leaks
        .into_raw(),
    }
}

pub extern "C" fn class_getName_v2(cls: ClassV2) -> *const c_char {
    match cls {
        None => std::ptr::null(),
        Some(cls) => CString::new(unsafe { cls.as_ref().name.as_str() })
            .expect("shouldn't have a null byte")
            .into_raw(),
    }
}

pub extern "C" fn class_getSuperClass(cls: Class) -> Class {
    unsafe {
        CONTEXT.lock().expect("poisoned mutex").classes[*cls?.as_ref()]
            .superclass
            .and_then(|class_key| NonNull::new(Box::into_raw(Box::new(class_key))))
    }
}

pub extern "C" fn class_getSuperClass_v2(cls: ClassV2) -> ClassV2 {
    let superclass = unsafe { cls?.as_ref() }.superclass?;
    NonNull::new(unsafe { &mut CONTEXT.lock().expect("poisoned mutex").classes[superclass] })
}

pub extern "C" fn class_isMetaClass(cls: Class) -> bool {
    match cls {
        None => false,
        Some(cls) => {
            match unsafe { &CONTEXT.lock().expect("poisoned mutex").class_kind[*cls.as_ref()] } {
                ClassKind::Meta => true,
                ClassKind::Regular => false,
            }
        }
    }
}

pub extern "C" fn class_getInstanceSize(cls: Class) -> libc::size_t {
    match cls {
        None => 0,
        Some(_cls) => unimplemented!("class_getInstanceSize"),
    }
}

pub extern "C" fn class_getInstanceVariable(
    cls: Class,
    name: *const c_char,
) -> Option<NonNull<Ivar>> {
    let name = unsafe { CStr::from_ptr(name) }
        .to_owned()
        .into_string()
        .expect("invalid utf8");

    unsafe {
        CONTEXT.lock().expect("poisoned mutex").classes[*cls?.as_ref()]
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
}

pub extern "C" fn class_getClassVariable(cls: Class, name: *const c_char) -> Option<NonNull<Ivar>> {
    let name = unsafe { CStr::from_ptr(name) }
        .to_owned()
        .into_string()
        .expect("invalid utf8");

    let mut context = unsafe { CONTEXT.lock().expect("poisoned mutex") };
    let metaclass = unsafe { context.classes[*cls?.as_ref()].metaclass };
    context.classes[metaclass]
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

pub extern "C" fn class_addIvar(
    cls: Class,
    name: *const c_char,
    size: usize,
    alignment: u8,
    types: *const c_char,
) -> bool {
    let cls = match cls {
        None => return false,
        Some(cls) => cls,
    };

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
        CONTEXT.lock().expect("poisoned mutex").classes[*cls.as_ref()].add_ivar(ivar)
    }
}
