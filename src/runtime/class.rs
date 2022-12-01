use std::{ffi::CString, ptr::NonNull};

use aligned_box::AlignedBox;

use super::{
    context::ClassKey,
    id,
    ivar::objc_ivar,
    message::Repr,
    method::Method,
    object::{objc_object, ObjectData},
    property::Property,
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
pub struct ClassData {
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
    pub methods: Vec<Method>,
    pub protocols: Vec<Protocol>,
    // TODO: this should be not an i8
    pub reference_list: i8,
    pub properties: Vec<Property>,
    pub info: Flags,
}

#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Default)]
pub struct objc_class(Repr<ClassData>);

impl std::ops::Deref for objc_class {
    type Target = Repr<ClassData>;

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
    pub fn new(class_key: ClassKey, class_data: ClassData) -> Self {
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

    pub fn create_object(&self) -> objc_object {
        let object_data = ObjectData {
            ivars: self
                .ivars
                .iter()
                .map(
                    |objc_ivar {
                         name,
                         size,
                         alignment,
                         ..
                     }| {
                        (
                            name.clone(),
                            AlignedBox::slice_from_default(alignment.to_uint(), *size)
                                .expect("invalid alignment"),
                        )
                    },
                )
                .collect(),
        };

        objc_object::new(self.is_a(), object_data)
    }
}

pub type Class = Option<std::ptr::NonNull<objc_class>>;
