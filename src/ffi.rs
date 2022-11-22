#![allow(non_snake_case)]

use crate::runtime::{
    class::{Class as ObjcClass, Flags},
    context::Context,
    ivar::Ivar,
    method::{Imp, Method},
    property::Property,
    selector::Selector,
};
use std::{
    cell::LazyCell,
    ffi::{c_char, c_uint, CStr},
    ptr::NonNull,
    sync::Mutex,
};

static mut CONTEXT: LazyCell<Mutex<Context>> = LazyCell::new(|| Mutex::new(Context::new()));

pub type Class = Option<NonNull<ObjcClass<'static>>>;

static EMPTY_STRING: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };

#[no_mangle]
pub extern "C" fn class_getName(cls: Class) -> *const c_char {
    match cls {
        None => EMPTY_STRING.as_ptr(),
        Some(cls) => unsafe { cls.as_ref() }.name.as_ptr(),
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

// TODO: rewrite using object_getClass
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

#[no_mangle]
pub extern "C" fn class_copyIvarList(
    cls: Class,
    out_count: *mut c_uint,
) -> Option<NonNull<NonNull<Ivar>>> {
    if !out_count.is_null() {
        unsafe { *out_count = 0 };
    }

    let ref mut ivars = unsafe { cls?.as_mut() }.ivars;

    if ivars.is_empty() {
        return None;
    }

    unsafe { *out_count = ivars.len() as c_uint };

    NonNull::new(
        Box::into_raw(
            ivars
                .iter_mut()
                .map(|ivar| unsafe { NonNull::new_unchecked(ivar as *mut Ivar) })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
        .as_mut_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn class_getIvarLayout(_cls: Class) -> *const u8 {
    unimplemented!("no garbage collection support yet")
}

#[no_mangle]
pub extern "C" fn class_setIvarLayout(_cls: Class, _layout: *const u8) {
    unimplemented!("no garbage collection support yet")
}

#[no_mangle]
pub extern "C" fn class_weakGetIvarLayout(_cls: Class) -> *const u8 {
    unimplemented!("no garbage collection support yet")
}

#[no_mangle]
pub extern "C" fn class_weakSetIvarLayout(_cls: Class, _layout: *const u8) {
    unimplemented!("no garbage collection support yet")
}

#[no_mangle]
pub extern "C" fn class_getProperty(cls: Class, name: *const c_char) -> Option<NonNull<Property>> {
    let mut cls = cls?;

    let name = unsafe { CStr::from_ptr(name) }
        .to_str()
        .expect("invalid utf8");

    let property = unsafe { cls.as_mut() }
        .properties
        .iter_mut()
        .find(|property| property.name == name)?;

    NonNull::new(property as *mut _)
}

// TODO: can we factor something out here to make this not duplicate [class_copyIvarList]?
#[no_mangle]
pub extern "C" fn class_copyPropertyList(
    cls: Class,
    out_count: *mut c_uint,
) -> Option<NonNull<NonNull<Property>>> {
    if !out_count.is_null() {
        unsafe { *out_count = 0 };
    }

    let ref mut properties = unsafe { cls?.as_mut() }.properties;

    if properties.is_empty() {
        return None;
    }

    unsafe { *out_count = properties.len() as c_uint };

    NonNull::new(
        Box::into_raw(
            properties
                .iter_mut()
                .map(|ivar| unsafe { NonNull::new_unchecked(ivar as *mut Property) })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
        .as_mut_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn class_addMethod(
    cls: Class,
    name: Option<NonNull<Selector>>,
    imp: Option<NonNull<Imp>>,
    types: *const c_char,
) -> bool {
    let x: Option<()> = try {
        let name = unsafe { name?.as_ref() };
        let imp = unsafe { imp?.as_ref() };
        let types = unsafe { CStr::from_ptr(types) }
            .to_owned()
            .into_string()
            .expect("invalid utf8");

        unsafe { cls?.as_mut() }
            .methods
            .push(Method::new(imp, name, types));
    };
    x.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_class_copyIvarList() {
        let mut objc_class = ObjcClass::default();
        let cls = NonNull::new(&mut objc_class as *mut _);
        let out_count = std::ptr::null::<c_uint>().cast_mut();
        let output = class_copyIvarList(cls, out_count);

        assert!(output.is_none());
        assert!(out_count.is_null());

        // we won't change [cls] at all, but now that we're supplying a pointer,
        // we should get zero back

        // just need this non-zero so we can test it's actually set
        let mut out_count: c_uint = 1;

        let new_output = class_copyIvarList(cls, &mut out_count as *mut _);

        assert_eq!(new_output, output);
        assert_eq!(out_count, 0);

        // add an ivar, see if we get a new number
        objc_class.ivars.push(Ivar::new(
            "fizzbuzz".to_string(),
            0,
            0,
            "foobar".to_string(),
        ));

        let new_output = class_copyIvarList(cls, &mut out_count as *mut _);
        assert!(new_output.is_some());
        assert_eq!(
            unsafe { new_output.unwrap().as_ref().as_ptr() } as *const _,
            &objc_class.ivars[0] as *const _
        );
        assert_eq!(out_count, 1);

        // the caller takes ownership of the returned pointers, so let's clean
        // up
        unsafe { Box::from_raw(new_output.unwrap().as_ptr()) };
    }
}
