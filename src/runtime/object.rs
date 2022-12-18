use std::alloc::{Allocator, Global, Layout};
use std::ptr::NonNull;

use super::context::ClassKey;
use super::message::Repr;

#[repr(C)]
pub struct ObjectData {
    /// This is not actually zero-sized: we need to do some yucky casting an make sure not to move this
    pub(crate) ivars: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(transparent)]
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
    pub fn new(class_key: ClassKey, dt_layout: Layout) -> NonNull<Self> {
        let (layout, dt_offset) = Layout::new::<Repr<ObjectData>>()
            .extend(dt_layout)
            .expect("bad layout I guess");

        assert_eq!(dt_offset, memoffset::offset_of!(Repr<ObjectData>, data));

        let mut obj: NonNull<Self> = Global
            .allocate_zeroed(layout)
            .expect("failed to allocate")
            .cast();

        // unsafe { std::mem::transmute(Global.allocate_zeroed(layout).expect("failed to allocate")) }
        unsafe { obj.as_mut() }.set__is_a(class_key);

        obj
    }
}
