use super::global_context::CONTEXT;

use crate::runtime::{id, method::IMP, Class, SEL};

use std::{
    ffi::{c_char, CStr},
    ptr::NonNull,
};

#[no_mangle]
pub extern "C" fn objc_allocateClassPair(
    superclass: Class,
    name: *const c_char,
    extra_bytes: libc::size_t,
) -> Class {
    println!("foobar");
    let name = unsafe { CStr::from_ptr(name) }.to_owned();
    let superclass = superclass.map(|superclass| unsafe { superclass.as_ref() }.index);

    let mut context = CONTEXT.write().expect("poisoned rwlock");
    context
        .allocate_class_pair(superclass, name, extra_bytes)
        .and_then(|class_key| NonNull::new(&mut context.classes[class_key] as *mut _))
}

#[no_mangle]
pub extern "C" fn objc_getClass(name: *const c_char) -> id {
    let name = unsafe { CStr::from_ptr(name) }.to_owned();
    let mut context = CONTEXT.write().expect("poisoned rwlock");
    let class_key = *context.registered_classes.get(&name)?;
    NonNull::new(&mut context.classes[class_key]).map(NonNull::cast)
}

#[no_mangle]
pub extern "C" fn objc_registerClassPair(cls: Class) {
    println!("registering class pair");
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

#[no_mangle]
pub extern "C" fn objc_getMetaClass(name: *const c_char) -> id {
    let name = unsafe { CStr::from_ptr(name) };
    let mut context = CONTEXT.write().expect("poisoned rwlock");
    let class_key = context.registered_classes.get(name)?;
    let metaclass_key = context.classes[*class_key].is_a();
    NonNull::new(&mut context.classes[metaclass_key] as *mut _).map(NonNull::cast)
}

pub extern "C" fn objc_msg_lookup(receiver: id, sel: SEL) -> IMP {
    let receiver = unsafe { receiver?.as_ref() };
    let sel = unsafe { sel?.as_ref() };
    CONTEXT.write().expect("poisoned rwlock").classes[**receiver]
        .methods
        .iter_mut()
        .find_map(|method| (method.selector == sel.index).then_some(method.imp))
}
