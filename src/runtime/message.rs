use std::ops::{Deref, DerefMut};

use super::context::ClassKey;

#[repr(transparent)]
#[derive(Default, Clone, Copy)]
pub struct Receiver(pub(crate) ClassKey);

#[repr(C)]
pub struct Repr<T> {
    /// Pointer to this object's class.
    pub is_a: Receiver,
    pub(crate) data: T,
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
