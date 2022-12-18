use std::alloc::{Allocator, Global, Layout};
use std::collections::HashMap;
use std::ptr::NonNull;

use aligned_box::AlignedBox;

use super::context::ClassKey;
use super::message::{id, Repr, ReprV2};

pub struct ObjectData {
    pub(crate) ivars: HashMap<String, AlignedBox<id>>,
}

#[repr(C)]
pub struct ObjectDataV2 {
    /// This is not actually zero-sized: we need to do some yucky casting an make sure not to move this
    pub(crate) ivars: [u8; 0],
}

#[repr(transparent)]
pub struct objc_object_v2(ReprV2<ObjectDataV2>);

impl std::ops::Deref for objc_object_v2 {
    type Target = ReprV2<ObjectDataV2>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for objc_object_v2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl objc_object_v2 {
    pub fn new(class_key: ClassKey, dt_layout: Layout) -> NonNull<Self> {
        let (layout, dt_offset) = Layout::new::<ReprV2<ObjectDataV2>>()
            .extend(dt_layout)
            .expect("bad layout I guess");

        assert_eq!(dt_offset, memoffset::offset_of!(ReprV2<ObjectDataV2>, data));

        let mut obj: NonNull<Self> = Global
            .allocate_zeroed(layout)
            .expect("failed to allocate")
            .cast();

        // unsafe { std::mem::transmute(Global.allocate_zeroed(layout).expect("failed to allocate")) }
        unsafe { obj.as_mut() }.set__is_a(class_key);

        obj
    }
}

pub struct objc_object(Repr<ObjectData>);

impl std::ops::Deref for objc_object {
    type Target = Repr<ObjectData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for objc_object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl objc_object {
    pub fn new(class_key: ClassKey, object_data: ObjectData) -> Self {
        Self(Repr::new(class_key, object_data))
    }
}
