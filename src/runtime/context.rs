use slotmap::{new_key_type, SecondaryMap, SlotMap};

new_key_type! {
    pub struct ClassKey;
}

use super::class::{Class, ClassKind, Flags};
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

    /// superclass: [None] if the class should be a root class
    pub fn allocate_class_pair<'a>(
        context: &'a mut Self,
        superclass: Option<ClassKey>,
        name: &str,
        _extra_bytes: usize,
    ) -> Option<ClassKey> {
        if context.registered_classes.contains_key(name) {
            return None;
        }

        let class_index = context.classes.insert(Class {
            superclass,
            ..Default::default()
        });
        let metaclass_index = context.classes.insert(Class::default());

        context.class_kind[class_index] = ClassKind::Regular;
        context.class_kind[metaclass_index] = ClassKind::Meta;

        match superclass {
            // Metaclasses of root classes are precious little flowers and work a
            // little differently
            None => {
                let metaclass = &mut context.classes[metaclass_index];
                metaclass.metaclass = metaclass_index;
                metaclass.superclass = Some(class_index);
            }
            Some(superclass_index) => {
                // TODO: do the superclass' need to be registered?
                let super_meta = context.classes.get(superclass_index)?.metaclass;
                let metaclass = &mut context.classes[metaclass_index];
                metaclass.metaclass = super_meta;
                metaclass.superclass = Some(super_meta);
            }
        }

        let metaclass = &mut context.classes[metaclass_index];
        metaclass.name = name.to_string();
        metaclass.info = Flags::USER_CREATED;

        // Set up the new class
        let class = &mut context.classes[class_index];
        class.metaclass = metaclass_index;
        class.superclass = superclass;

        class.name = name.to_string();
        class.info = Flags::USER_CREATED;

        Some(class_index)
    }
}
