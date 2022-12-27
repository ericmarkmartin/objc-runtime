use super::empty_string::EMPTY_STRING;
use super::global_context::CONTEXT;
use crate::runtime::{
    id,
    ivar::{objc_ivar, Ivar},
    method::{objc_method, Method, IMP},
    property::Property,
    Class, SEL,
};
use std::{
    ffi::{c_char, c_uint, CStr},
    ptr::NonNull,
};

#[no_mangle]
pub unsafe extern "C" fn class_getClassMethod(cls: Class, name: SEL) -> Method {
    let cls = cls?.as_ref();
    let name = name?.as_ref();
    let mut context = CONTEXT.write().expect("poisoned rwlock");
    let method = context.classes[cls.is_a()]
        .methods
        .iter_mut()
        .find_map(|method| (method.selector == name.index).then_some(method))?;

    NonNull::new(method as *mut _)
}

#[no_mangle]
pub unsafe extern "C" fn class_getInsatnceMethod(cls: Class, name: SEL) -> Method {
    let cls = cls?.as_mut();
    let name = name?.as_ref();

    let method = cls
        .methods
        .iter_mut()
        .find_map(|method| (method.selector == name.index).then_some(method))?;

    NonNull::new(method as *mut _)
}

/// Returns a Boolean value that indicates whether instances of a class respond
/// to a particular selector.
pub unsafe fn class_respondsToSelector(cls: Class, sel: SEL) -> bool {
    let cls = match cls {
        Some(cls) => cls.as_ref(),
        None => return false,
    };

    let sel = match sel {
        Some(sel) => sel.as_ref(),
        None => return false,
    };

    cls.methods
        .iter()
        .any(|method| method.selector == sel.index)
}

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
    NonNull::new(&mut CONTEXT.write().expect("poisoned rwlock").classes[superclass])
}

#[no_mangle]
pub extern "C" fn class_isMetaClass(cls: Class) -> bool {
    match cls {
        None => false,
        Some(cls) => unsafe { cls.as_ref() }.is_metaclass(),
    }
}

#[no_mangle]
pub extern "C" fn class_getInstanceSize(cls: Class) -> libc::size_t {
    println!("getting instance size: {cls:?}");
    match cls {
        None => 0,
        Some(cls) => unsafe { cls.as_ref() }.instance_layout().size(),
    }
}

#[no_mangle]
pub extern "C" fn class_getInstanceVariable(cls: Class, name: *const c_char) -> Ivar {
    println!("getting instance variable");
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
pub extern "C" fn class_getClassVariable(cls: Class, name: *const c_char) -> Ivar {
    let name = unsafe { CStr::from_ptr(name) }
        .to_owned()
        .into_string()
        .expect("invalid utf8");

    CONTEXT.write().expect("poisoned rwlock").classes[unsafe { cls?.as_ref() }.is_a()]
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
            let ivar = objc_ivar::new(name, size, alignment, types);
            unsafe { cls.as_mut() }.add_ivar(ivar)
        }
    }
}

#[no_mangle]
pub extern "C" fn class_copyIvarList(
    cls: Class,
    out_count: *mut c_uint,
) -> Option<NonNull<NonNull<crate::runtime::ivar::objc_ivar>>> {
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
                .map(|ivar| unsafe { NonNull::new_unchecked(ivar as *mut _) })
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
pub extern "C" fn class_addMethod(cls: Class, name: SEL, imp: IMP, types: *const c_char) -> bool {
    let x: Option<()> = try {
        let name = unsafe { name?.as_ref() };
        let imp = imp?;
        let cls = unsafe { cls?.as_mut() };
        let types = unsafe { CStr::from_ptr(types) }
            .to_owned()
            .into_string()
            .expect("invalid utf8");

        cls.methods.push(objc_method::new(imp, name, types));
    };
    x.is_some()
}

// TODO: match casing on (e.g.) [extra_bytes]
#[no_mangle]
pub extern "C" fn class_createInstance(cls: Class, _extra_bytes: libc::size_t) -> id {
    // TODO: add [extra_bytes] to the layout
    Some(unsafe { cls?.as_ref() }.create_object().cast())
}
