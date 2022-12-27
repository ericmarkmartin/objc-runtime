#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod class;
mod empty_string;
mod global_context;
pub mod objc;
pub mod object;
pub mod sel;

pub use class::*;
pub use objc::*;
pub use object::*;
pub use sel::*;

// TODO: null-check name pointers

#[cfg(test)]
mod tests {
    use empty_string::EMPTY_STRING;

    use crate::runtime::{class::Class, id, selector::SEL};
    use std::ffi::{c_uint, CString};
    use std::ptr::NonNull;

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
        let sel = unsafe { sel_registerName(sel_name.as_ptr()) };

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
        let cls_name = CString::new("foobar4").expect("valid utf8");
        let cls = objc_allocateClassPair(None, cls_name.as_ptr(), 0);

        assert!(cls.is_some());

        objc_registerClassPair(cls);

        let ivar_name = CString::new("fizzbuzz").expect("valid utf8");
        // TODO: fill in the types
        class_addIvar(
            cls,
            ivar_name.as_ptr(),
            std::mem::size_of::<id>(),
            std::mem::size_of::<id>().ilog2() as u8,
            EMPTY_STRING.as_ptr(),
        );

        let ivar = class_getInstanceVariable(cls, ivar_name.as_ptr());

        let obj = class_createInstance(cls, 0);
        assert!(obj.is_some());
        let obj2 = class_createInstance(cls, 0);
        assert!(obj2.is_some());

        assert_eq!(object_getIvar(obj, ivar), None);

        object_setIvar(obj, ivar, obj2);

        let new_value = object_getIvar(obj, ivar);

        assert_eq!(new_value, obj2);
    }
}
