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

#[repr(C)]
#[derive(Default)]
/// cbindgen:ignore
pub struct Repr<T: ?Sized> {
    /// Pointer to this object's class.
    is_a: Receiver,
    pub(crate) data: T,
}

impl<T> Repr<T> {
    pub(crate) fn new(is_a: ClassKey, data: T) -> Self {
        Self {
            is_a: Receiver::new(is_a),
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

#[allow(non_camel_case_types)]
pub type id = Option<std::ptr::NonNull<Receiver>>;

// TODO: we should have
// ```
// impl From<Class> for id {
//   ...
// }
// ```
// here but we can't until we make these tuple structs
