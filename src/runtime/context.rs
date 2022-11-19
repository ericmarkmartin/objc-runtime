use super::class::{Class, ClassPair};
use std::rc::Rc;

pub struct ClassTable(pub Vec<ClassPair>);

impl ClassTable {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn lookup(&self, name: &str) -> Option<&ClassPair> {
        self.0.iter().find(
            |ClassPair {
                 class,
                 metaclass: _,
             }| class.0.borrow().name == *name,
        )
    }

    pub fn push(&mut self, class: Rc<Class>, metaclass: Rc<Class>) {
        self.0.push(ClassPair { class, metaclass });
    }
}

pub struct Context {
    pub(crate) registered_classes: ClassTable,
    pub(crate) unregistered_classes: ClassTable,
}

impl Context {
    pub const fn new() -> Self {
        Self {
            registered_classes: ClassTable::new(),
            unregistered_classes: ClassTable::new(),
        }
    }
}
