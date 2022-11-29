use std::ops::{Deref, DerefMut};

use super::context::ClassKey;

#[repr(transparent)]
#[derive(Default, Clone, Copy)]
pub struct Receiver(ClassKey);

impl Receiver {
    pub fn new(class_key: ClassKey) -> Self {
        Self(class_key)
    }
}

impl Deref for Receiver {
    type Target = ClassKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// cbindgen:ignore
#[repr(C)]
pub struct Repr<T> {
    /// Pointer to this object's class.
    is_a: Receiver,
    data: T,
}

impl<T> Repr<T> {
    pub fn new(class_key: ClassKey, data: T) -> Self {
        Repr {
            is_a: Receiver::new(class_key),
            data,
        }
    }

    pub fn set__is_a(&mut self, class_key: ClassKey) {
        self.is_a = Receiver::new(class_key);
    }
    pub const fn is_a(&self) -> ClassKey {
        self.is_a.0
    }
}

impl<T> Deref for Repr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Repr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Default> Default for Repr<T> {
    fn default() -> Self {
        Self {
            is_a: Default::default(),
            data: Default::default(),
        }
    }
}

#[allow(non_camel_case_types)]
type objc_object = Receiver;

pub type id = Option<std::ptr::NonNull<objc_object>>;
