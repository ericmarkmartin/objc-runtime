use std::collections::HashMap;

use aligned_box::AlignedBox;

use super::context::ClassKey;
use super::message::Repr;

pub struct ObjectData {
    pub(crate) ivars: HashMap<String, AlignedBox<[u8]>>,
}

pub struct objc_object(Repr<ObjectData>);

impl std::ops::Deref for objc_object {
    type Target = Repr<ObjectData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for objc_object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl objc_object {
    pub fn new(class_key: ClassKey, object_data: ObjectData) -> Self {
        Self(Repr::new(class_key, object_data))
    }
}
