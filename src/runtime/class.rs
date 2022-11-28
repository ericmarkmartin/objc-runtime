use std::ffi::CString;

use super::{
    context::ClassKey, ivar::Ivar, message::Repr, method::Method, property::Property,
    protocol::Protocol,
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

#[derive(Default)]
pub struct ClassData<'a> {
    pub superclass: Option<ClassKey>,
    // TODO: this should be not an i8
    // dispatch_table: i8,
    // first_subclass: Arc<Class>,
    // cxx_construct: Option<Imp>,
    // cxx_destruct: Option<Imp>,
    // first_sibling: Box<Class>,
    /// We use a [CString] because in [class_getName] we need to present a
    /// C-compatible (null-terminated) string and we need somewhere to store the
    /// string data w/ the null byte.
    pub(crate) name: CString,
    pub(crate) index: ClassKey,
    pub ivars: Vec<Ivar>,
    pub methods: Vec<Method<'a>>,
    pub protocols: Vec<Protocol>,
    // TODO: this should be not an i8
    pub reference_list: i8,
    pub properties: Vec<Property>,
    pub info: Flags,
}

impl Repr<ClassData<'_>> {
    fn is_registered(&self) -> bool {
        // TODO: implement
        return true;
    }

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
