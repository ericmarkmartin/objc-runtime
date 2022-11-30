use super::{
    message::id,
    selector::{Selector, SEL},
};

pub type objc_imp = unsafe extern "C" fn(id, SEL, ...);
pub type IMP = Option<objc_imp>;

pub struct Method<'a> {
    pub(crate) imp: objc_imp,
    pub(crate) selector: &'a Selector,
    types: String,
}

impl<'a> Method<'a> {
    pub fn new(imp: objc_imp, selector: &'a Selector, types: String) -> Self {
        Self {
            imp,
            selector,
            types,
        }
    }
}
