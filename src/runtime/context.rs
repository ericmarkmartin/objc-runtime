use slotmap::{new_key_type, SecondaryMap, SlotMap};

new_key_type! {
    pub struct ClassKey;
}

use super::class::{Class, ClassKind};
use std::collections::HashMap;

pub struct Context {
    pub(crate) classes: SlotMap<ClassKey, Class>,
    pub(crate) registered_classes: HashMap<String, ClassKey>,
    pub(crate) registered_metaclasses: HashMap<String, ClassKey>,
    pub(crate) class_kind: SecondaryMap<ClassKey, ClassKind>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            classes: SlotMap::with_key(),
            registered_classes: HashMap::new(),
            registered_metaclasses: HashMap::new(),
            class_kind: SecondaryMap::new(),
        }
    }
}
