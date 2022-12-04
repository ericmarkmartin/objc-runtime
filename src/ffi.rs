#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use aligned_box::AlignedBox;
use libc::c_void;
use std::{ptr, sync::LazyLock};

use super::runtime::{
    context::Context,
    id,
    ivar::{objc_ivar, Ivar},
    message::Receiver,
    method::{Method, IMP},
    object::objc_object,
    property::Property,
    selector::Selector,
    Class, SEL,
};
use std::{
    ffi::{c_char, c_uint, CStr},
    ptr::NonNull,
    sync::RwLock,
};

static CONTEXT: LazyLock<RwLock<Context>> = LazyLock::new(|| RwLock::new(Context::new()));

static EMPTY_STRING: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };

// TODO: null-check name pointers

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
    match cls {
        None => 0,
        Some(_cls) => unimplemented!("class_getInstanceSize"),
    }
}

#[no_mangle]
pub extern "C" fn class_getInstanceVariable(cls: Class, name: *const c_char) -> Ivar {
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

        cls.methods.push(Method::new(imp, name, types));
    };
    x.is_some()
}

#[no_mangle]
pub extern "C" fn objc_allocateClassPair(
    superclass: Class,
    name: *const c_char,
    extra_bytes: libc::size_t,
) -> Class {
    let name = unsafe { CStr::from_ptr(name) }.to_owned();
    let superclass = superclass.map(|superclass| unsafe { superclass.as_ref() }.index);

    let mut context = CONTEXT.write().expect("poisoned rwlock");
    context
        .allocate_class_pair(superclass, name, extra_bytes)
        .and_then(|class_key| NonNull::new(&mut context.classes[class_key] as *mut _))
}

#[no_mangle]
pub extern "C" fn objc_registerClassPair(cls: Class) {
    match cls {
        None => (),
        Some(cls) => {
            let cls = unsafe { cls.as_ref() };
            CONTEXT
                .write()
                .expect("poisoned rwlock")
                .registered_classes
                .insert(cls.name.clone(), cls.index);
        }
    }
}

// TODO: match casing on (e.g.) [extra_bytes]
#[no_mangle]
pub extern "C" fn class_createInstance(cls: Class, _extra_bytes: libc::size_t) -> id {
    let object = unsafe { cls?.as_ref() }.create_object();
    NonNull::new(Box::into_raw(Box::new(object))).map(NonNull::cast)
}

// TODO: think about how to do this with primitively-typed ivars.
#[no_mangle]
pub extern "C" fn object_getIvar(obj: id, ivar: Ivar) -> id {
    let ivar = unsafe { ivar?.as_ref() };
    let aligned_box = &unsafe { obj?.cast::<objc_object>().as_mut() }.ivars[&ivar.name];

    **unsafe { std::mem::transmute::<_, &AlignedBox<id>>(aligned_box) }
}

#[no_mangle]
pub extern "C" fn object_setIvar(obj: id, ivar: Ivar, value: id) {
    let _: Option<()> = try {
        let ivar = unsafe { ivar?.as_ref() };
        let aligned_box = unsafe { obj?.cast::<objc_object>().as_mut() }
            .ivars
            .get_mut(&ivar.name)
            .expect("ivar wasn't there");

        **unsafe { std::mem::transmute::<_, &mut AlignedBox<id>>(aligned_box) } = value;
    };
}

#[no_mangle]
pub extern "C" fn object_getInstanceVariable(
    obj: id,
    name: *const c_char,
    out_value: *mut *mut c_void,
) -> Ivar {
    let ivar: Ivar = {
        let obj = unsafe { obj?.as_ref() };
        let name = unsafe { CStr::from_ptr(name) }
            .to_owned()
            .into_string()
            .expect("invalid utf8");
        NonNull::new(
            CONTEXT.write().expect("poisoned rwlock").classes[**obj]
                .ivars
                .iter_mut()
                .find(|objc_ivar { name: name_, .. }| &name == name_)?,
        )
    };

    unsafe {
        *out_value = match object_getIvar(obj, ivar) {
            None => ptr::null_mut(),
            Some(nonnull) => nonnull.cast().as_mut(),
        }
    };

    ivar
}

#[no_mangle]
pub extern "C" fn object_setInstanceVariable(
    obj: id,
    name: *const c_char,
    value: *mut c_void,
) -> Ivar {
    let ivar: Ivar = {
        let obj = unsafe { obj?.as_ref() };
        let name = unsafe { CStr::from_ptr(name) }
            .to_owned()
            .into_string()
            .expect("invalid utf8");
        NonNull::new(
            CONTEXT.write().expect("poisoned rwlock").classes[**obj]
                .ivars
                .iter_mut()
                .find(|objc_ivar { name: name_, .. }| &name == name_)?,
        )
    };

    object_setIvar(obj, ivar, NonNull::new(value as *mut Receiver));

    ivar
}

#[no_mangle]
pub extern "C" fn objc_getMetaClass(name: *const c_char) -> id {
    let name = unsafe { CStr::from_ptr(name) };
    let mut context = CONTEXT.write().expect("poisoned rwlock");
    let class_key = context.registered_classes.get(name)?;
    let metaclass_key = context.classes[*class_key].is_a();
    NonNull::new(&mut context.classes[metaclass_key] as *mut _).map(NonNull::cast)
}

#[no_mangle]
pub extern "C" fn sel_registerName(name: *const c_char) -> Option<NonNull<Selector>> {
    let name = unsafe { CStr::from_ptr(name) }
        .to_owned()
        .into_string()
        .expect("invalid utf8");
    let mut context = CONTEXT.write().expect("poisoned rwlock");
    let selector_key = context.allocate_selector(name);
    NonNull::new(&mut context.selectors[selector_key] as *mut _)
}

pub extern "C" fn objc_msg_lookup(receiver: id, sel: SEL) -> IMP {
    let receiver = unsafe { receiver?.as_ref() };
    let sel = unsafe { sel?.as_ref() };
    CONTEXT.write().expect("poisoned rwlock").classes[**receiver]
        .methods
        .iter_mut()
        .find_map(|method| (method.selector == sel.index).then_some(method.imp))
}

#[cfg(test)]
mod tests {

    use std::ffi::CString;

    use super::*;
    #[test]
    fn test_class_copyIvarList() {
        let out_count = std::ptr::null::<c_uint>().cast_mut();
        let output = class_copyIvarList(None, out_count);
        assert!(output.is_none());

        let name = CString::new("foobar").expect("valid utf8");
        let cls = objc_allocateClassPair(None, name.as_ptr(), 0);
        let output = class_copyIvarList(cls, out_count);
        assert!(output.is_none());

        // we won't change [cls] at all, but now that we're supplying a pointer,
        // we should get zero back

        // just need this non-zero so we can test it's actually set
        let mut out_count: c_uint = 1;

        let new_output = class_copyIvarList(cls, &mut out_count as *mut _);

        assert_eq!(new_output, output);
        assert_eq!(out_count, 0);

        // add an ivar, see if we get a new number
        let ivar_name = CString::new("fizzbuzz").expect("valid utf8");
        class_addIvar(cls, ivar_name.as_ptr(), 0, 0, EMPTY_STRING.as_ptr());

        let new_output = class_copyIvarList(cls, &mut out_count as *mut _);
        assert!(new_output.is_some());
        assert_eq!(out_count, 1);

        // the caller takes ownership of the returned pointers, so let's clean
        // up
        unsafe { Box::from_raw(new_output.unwrap().as_ptr()) };
    }

    #[test]
    fn test_send_message() {
        let cls_name = CString::new("foobar2").expect("valid utf8");
        let cls = objc_allocateClassPair(None, cls_name.as_ptr(), 0);

        objc_registerClassPair(cls);

        // TODO: we should expose functions for these casts
        let metaclass: Class = objc_getMetaClass(cls_name.as_ptr()).map(NonNull::cast);

        let sel_name = CString::new("fizzbuzz").expect("valid utf8");
        let sel = sel_registerName(sel_name.as_ptr());

        unsafe extern "C" fn imp(self_: id, _cmd: SEL, ...) -> id {
            // TODO: do something with [_cmd] to make sure we're passing it correctly.
            self_
        }

        // TODO: types should actually be something
        let types = EMPTY_STRING.as_ptr();

        assert!(class_addMethod(metaclass, sel, Some(imp), types));

        let id = cls.map(NonNull::cast);

        let imp = objc_msg_lookup(id, sel).expect("should be a real function");

        assert_eq!(id, unsafe { imp(id, sel) });
    }

    #[test]
    fn test_get_set_ivar() {
        let cls_name = CString::new("foobar3").expect("valid utf8");
        let cls = objc_allocateClassPair(None, cls_name.as_ptr(), 0);

        objc_registerClassPair(cls);

        let ivar_name = CString::new("fizzbuzz").expect("valid utf8");
        // TODO: fill in the types
        class_addIvar(
            cls,
            ivar_name.as_ptr(),
            std::mem::size_of::<id>(),
            0,
            EMPTY_STRING.as_ptr(),
        );

        let ivar = class_getInstanceVariable(cls, ivar_name.as_ptr());

        let obj = class_createInstance(cls, 0);
        let obj2 = class_createInstance(cls, 0);

        assert!(object_getIvar(obj, ivar).is_none());

        object_setIvar(obj, ivar, obj2);

        let new_value = object_getIvar(obj, ivar);

        assert_eq!(new_value, obj2);
    }
}
