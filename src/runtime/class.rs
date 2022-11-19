use super::{context::Context, ivar::Ivar, method::Method, property::Property, protocol::Protocol};
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    rc::{Rc, Weak},
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
pub struct ClassData {
    metaclass: Weak<Class>,
    superclass: Option<Weak<Class>>,
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

pub struct Class(pub(crate) RefCell<ClassData>);

pub struct ClassPair {
    pub(crate) class: Rc<Class>,
    pub(crate) metaclass: Rc<Class>,
}

impl Class {
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
            .0
            .borrow()
            .ivars
            .iter()
            .find(|ivar_| ivar.name == ivar_.name)
            .is_some()
        {
            return false;
        }

        self.0.borrow_mut().ivars.push(ivar);
        true
    }

    /// superclass: [None] if the class should be a root class
    pub fn alloc<'a>(
        context: &'a mut Context,
        superclass: Option<&'a Class>,
        name: &str,
        _extra_bytes: usize,
    ) -> Option<&'a Self> {
        if context.registered_classes.lookup(name).is_some() {
            return None;
        }

        let class = Rc::new(Class(RefCell::new(ClassData::default())));
        let metaclass = Rc::new(Class(RefCell::new(ClassData::default())));

        {
            let mut metaclass_ref = metaclass.0.borrow_mut();
            match superclass {
                // Metaclasses of root classes are precious little flowers and work a
                // little differently
                None => {
                    metaclass_ref.metaclass = Rc::downgrade(&metaclass);
                    metaclass_ref.superclass = Some(Rc::downgrade(&class));
                }
                Some(superclass) => {
                    // Initialize the metaclass
                    // Set the meta-metaclass pointer to the name.  The runtime will fix this
                    // in objc_resolve_class().
                    // If the superclass is not yet resolved, then we need to look it up
                    // via the class table.

                    let super_meta = &superclass.0.borrow().metaclass;

                    metaclass_ref.metaclass = super_meta.clone();
                    metaclass_ref.superclass = Some(super_meta.clone());
                }
            }
        };

        context
            .unregistered_classes
            .0
            .push(ClassPair { class, metaclass });
        Some(
            &*context
                .unregistered_classes
                .0
                .last()
                .expect("just added an element")
                .class,
        )
    }
}
