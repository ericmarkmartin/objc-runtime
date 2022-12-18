use slotmap::{new_key_type, SlotMap};

new_key_type! {
    pub struct ClassKey;
}

new_key_type! {
    pub struct SelectorKey;
}

use super::{
    class::{objc_class, ClassData, Flags},
    selector::{objc_selector, SelectorInfo},
};
use std::{collections::HashMap, ffi::CString};

pub struct Context {
    pub(crate) classes: SlotMap<ClassKey, objc_class>,
    pub(crate) selectors: SlotMap<SelectorKey, objc_selector>,
    pub(crate) registered_classes: HashMap<CString, ClassKey>,
    pub(crate) registered_metaclasses: HashMap<CString, ClassKey>,
    pub(crate) selectors_by_name: HashMap<SelectorInfo, SelectorKey>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            classes: SlotMap::with_key(),
            selectors: SlotMap::with_key(),
            registered_classes: HashMap::new(),
            registered_metaclasses: HashMap::new(),
            selectors_by_name: HashMap::new(),
        }
    }

    /// superclass: [None] if the class should be a root class
    pub fn allocate_class_pair<'a>(
        &mut self,
        superclass: Option<ClassKey>,
        name: CString,
        extra_bytes: usize,
    ) -> Option<ClassKey> {
        if self.registered_classes.contains_key(&name) {
            return None;
        }

        let class_index = self.classes.insert_with_key(|index| {
            objc_class::new(
                Default::default(),
                ClassData {
                    superclass,
                    index,
                    extra_bytes,
                    ..Default::default()
                },
            )
        });

        let metaclass_index = self.classes.insert(objc_class::default());

        match superclass {
            // Metaclasses of root classes are precious little flowers and work a
            // little differently
            None => {
                let metaclass = &mut self.classes[metaclass_index];
                metaclass.set__is_a(metaclass_index);
                metaclass.superclass = Some(class_index);
            }
            Some(superclass_index) => {
                // TODO: do the superclass' need to be registered?
                let super_meta = self.classes.get(superclass_index)?.is_a();
                let metaclass = &mut self.classes[metaclass_index];
                metaclass.set__is_a(super_meta);
                metaclass.superclass = Some(super_meta);
            }
        }

        let metaclass = &mut self.classes[metaclass_index];
        metaclass.name = name.clone();
        metaclass.info = Flags::USER_CREATED | Flags::META;

        // Set up the new class
        let class = &mut self.classes[class_index];
        class.set__is_a(metaclass_index);
        class.superclass = superclass;

        class.name = name;
        class.info = Flags::USER_CREATED;

        Some(class_index)
    }

    pub fn allocate_selector(&mut self, name: String) -> SelectorKey {
        // If an identical selector is already registered, return it.
        let selector_info = SelectorInfo::new(name);
        *self
            .selectors_by_name
            .entry(selector_info)
            .or_insert_with_key(|selector_info| {
                self.selectors.insert_with_key(|index| objc_selector {
                    selector_info: selector_info.clone(),
                    index,
                })
            })
    }
}
