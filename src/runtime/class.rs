use slotmap::Key;

use super::{
    context::{ClassKey, Context},
    ivar::Ivar,
    method::Method,
    property::Property,
    protocol::Protocol,
};

bitflags::bitflags! {
    struct Flags: usize {
        const META = 0b00000001;
        const USER_CREATED = 0b00000010;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::empty()
    }
}

pub trait Receiver {
    fn is_a(&self) -> Option<&dyn Receiver>;
}

#[derive(Default)]
pub struct Class {
    pub metaclass: ClassKey,
    pub superclass: Option<ClassKey>,
    // TODO: this should be not an i8
    // dispatch_table: i8,
    // first_subclass: Arc<Class>,
    // cxx_construct: Option<Imp>,
    // cxx_destruct: Option<Imp>,
    // first_sibling: Box<Class>,
    pub(crate) name: String,
    ivars: Vec<Ivar>,
    methods: Vec<Method>,
    protocols: Vec<Protocol>,
    // TODO: this should be not an i8
    reference_list: i8,
    properties: Vec<Property>,
}

pub enum ClassKind {
    Regular,
    Meta,
}

impl Class {
    pub fn new(name: &str, metaclass: ClassKey, superclass: Option<ClassKey>) -> Self {
        Self {
            metaclass,
            superclass,
            name: name.to_string(),
            ivars: Vec::new(),
            methods: Vec::new(),
            protocols: Vec::new(),
            reference_list: 0,
            properties: Vec::new(),
        }
    }

    fn is_registered(&self) -> bool {
        // TODO: implement
        return true;
    }

    // fn get_ivar_with_name(&self, name: &str) -> Option<&Ivar> {
    //     self.0.borrow().ivars.iter().find(|ivar| ivar.name == name)
    // }

    pub fn add_ivar(&mut self, ivar: Ivar) -> bool {
        // Class must already be registered
        if !self.is_registered() {
            return false;
        }

        // No duplicate ivar names
        if !self
            .ivars
            .iter()
            .find(|ivar_| ivar.name == ivar_.name)
            .is_some()
        {
            return false;
        }

        self.ivars.push(ivar);
        true
    }

    /// superclass: [None] if the class should be a root class
    pub fn alloc<'a>(
        context: &'a mut Context,
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

        Some(class_index)
    }
}
