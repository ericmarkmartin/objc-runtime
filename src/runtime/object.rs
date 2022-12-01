use std::collections::HashMap;

use aligned_box::AlignedBox;

use super::{context::ClassKey, message::Repr};

pub struct ObjectData {
    pub(crate) ivars: HashMap<String, AlignedBox<[u8]>>,
}

pub struct objc_object(Repr<ObjectData>);

impl objc_object {
    pub fn new(class_key: ClassKey, object_data: ObjectData) -> Self {
        Self(Repr::new(class_key, object_data))
    }
}
