use super::{
    context::SelectorKey,
    message::id,
    selector::{Selector, SEL},
};

pub type objc_imp = unsafe extern "C" fn(id, SEL, ...) -> id;
pub type IMP = Option<objc_imp>;

pub struct Method {
    pub(crate) imp: objc_imp,
    pub(crate) selector: SelectorKey,
    types: String,
}

impl Method {
    pub fn new(imp: objc_imp, selector: &Selector, types: String) -> Self {
        Self {
            imp,
            selector: selector.index,
            types,
        }
    }
}
