use std::ffi::CString;

use super::{
    context::ClassKey, ivar::objc_ivar, message::Repr, method::Method, property::Property,
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

/// cbindgen:ignore
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
    pub ivars: Vec<objc_ivar>,
    pub methods: Vec<Method<'a>>,
    pub protocols: Vec<Protocol>,
    // TODO: this should be not an i8
    pub reference_list: i8,
    pub properties: Vec<Property>,
    pub info: Flags,
}

#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Default)]
pub struct objc_class(Repr<ClassData<'static>>);

impl std::ops::Deref for objc_class {
    type Target = Repr<ClassData<'static>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for objc_class {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl objc_class {
    pub fn new(class_key: ClassKey, class_data: ClassData<'static>) -> Self {
        Self(Repr::new(class_key, class_data))
    }

    fn is_registered(&self) -> bool {
        // TODO: make this do an actual thing
        true
    }

    pub(crate) fn is_metaclass(&self) -> bool {
        self.info.contains(Flags::META)
    }

    pub fn add_ivar(&mut self, ivar: objc_ivar) -> bool {
        // Class must already be registered
        if !self.is_registered() {
            return false;
        }

        // No duplicate ivar names
        if self
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

pub type Class = Option<std::ptr::NonNull<objc_class>>;
