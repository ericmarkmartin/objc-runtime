use super::{
    context::ClassKey, ivar::Ivar, method::Method, property::Property, protocol::Protocol,
};

bitflags::bitflags! {
    pub struct Flags: usize {
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
    pub ivars: Vec<Ivar>,
    pub methods: Vec<Method>,
    pub protocols: Vec<Protocol>,
    // TODO: this should be not an i8
    pub reference_list: i8,
    pub properties: Vec<Property>,
    pub info: Flags,
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
            info: Flags::default(),
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
}
