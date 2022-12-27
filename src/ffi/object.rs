use super::global_context::CONTEXT;
use crate::runtime::{
    class::Class,
    id,
    ivar::{objc_ivar, Ivar},
    message::Receiver,
    object::objc_object,
};
use libc::ptrdiff_t;
use std::ffi::{c_char, c_void, CStr};
use std::ptr::NonNull;

#[no_mangle]
pub extern "C" fn object_getIvar(obj: id, ivar: Ivar) -> id {
    unsafe {
        let ivar = { ivar?.as_ref() };
        std::ptr::read(
            (&obj?.cast::<objc_object>().as_mut().ivars[ivar.offset..]).as_ptr() as *const _,
        )
    }
}

#[no_mangle]
pub extern "C" fn object_getClass(obj: id) -> Class {
    let class_key = unsafe { obj?.as_ref() };
    NonNull::new(&mut CONTEXT.write().expect("poisoned rwlock").classes[**class_key])
}

#[no_mangle]
pub extern "C" fn ivar_getOffset(ivar: Ivar) -> ptrdiff_t {
    if let Some(ivar) = ivar {
        unsafe { ivar.as_ref() }.offset as ptrdiff_t
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn object_setIvar(obj: id, ivar: Ivar, value: id) {
    let _: Option<()> = try {
        let ivar = unsafe { ivar?.as_ref() };
        let ivar = &mut unsafe { obj?.cast::<objc_object>().as_mut() }.ivars[ivar.offset..];

        unsafe { std::ptr::write(ivar.as_mut_ptr() as *mut _, value) };
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
            None => std::ptr::null_mut(),
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
