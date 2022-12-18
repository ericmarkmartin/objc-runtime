use std::{alloc::Layout, ffi::CString, ptr::NonNull};

use aligned_box::AlignedBox;

use super::{
    context::ClassKey,
    id,
    ivar::objc_ivar,
    message::{Repr, ReprV2},
    method::Method,
    object::{objc_object, objc_object_v2, ObjectData, ObjectDataV2},
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
    pub(crate) ivar_layout: Option<std::alloc::Layout>,
    pub(crate) extra_bytes: usize,
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

    pub fn add_ivar(&mut self, mut ivar: objc_ivar) -> bool {
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

        let new_layout = std::alloc::Layout::from_size_align(ivar.size, ivar.alignment.to_uint())
            .expect("invalid size-alignment combination");
        let (ivar_layout, offset) = match self.ivar_layout {
            Some(layout) => layout.extend(new_layout).expect(""),
            None => (new_layout, 0),
        };

        ivar.offset = offset;
        self.ivar_layout = Some(ivar_layout);

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
                         size: _,
                         alignment,
                         ..
                     }| {
                        (
                            name.clone(),
                            AlignedBox::new(alignment.to_uint(), None).expect("invalid alignment"),
                        )
                    },
                )
                .collect(),
        };

        objc_object::new(self.is_a(), object_data)
    }

    pub(crate) fn instance_layout(&self) -> Layout {
        let extra_bytes_layout =
            Layout::from_size_align(self.extra_bytes, std::mem::align_of::<u8>())
                .expect("invalid size/align");
        let dtable_layout = match self.ivar_layout {
            Some(ivar_layout) => {
                let (layout, _extra_bytes_offset) = ivar_layout
                    .extend(extra_bytes_layout)
                    .expect("invalid layout extension");
                // TODO: store the extra bytes offset
                layout
            }
            None => extra_bytes_layout,
        };
        let (layout, _dt_offset) = Layout::new::<ReprV2<ObjectDataV2>>()
            .extend(dtable_layout)
            .expect("bad layout I guess");
        layout
    }

    pub fn create_object_v2(&self) -> NonNull<objc_object_v2> {
        let extra_bytes_layout =
            Layout::from_size_align(self.extra_bytes, std::mem::align_of::<u8>())
                .expect("invalid size/align");
        let dtable_layout = match self.ivar_layout {
            Some(ivar_layout) => {
                let (layout, _extra_bytes_offset) = ivar_layout
                    .extend(extra_bytes_layout)
                    .expect("invalid layout extension");
                // TODO: store the extra bytes offset
                layout
            }
            None => extra_bytes_layout,
        };
        objc_object_v2::new(self.is_a(), dtable_layout)
    }
}

pub type Class = Option<std::ptr::NonNull<objc_class>>;
