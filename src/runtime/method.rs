use super::{
    context::SelectorKey,
    message::id,
    selector::{objc_selector, SEL},
};

pub type objc_imp = unsafe extern "C" fn(id, SEL, ...) -> id;
pub type IMP = Option<objc_imp>;

#[allow(non_camel_case_types)]
pub struct objc_method {
    pub(crate) imp: objc_imp,
    pub(crate) selector: SelectorKey,
    types: String,
}

impl objc_method {
    pub fn new(imp: objc_imp, selector: &objc_selector, types: String) -> Self {
        Self {
            imp,
            selector: selector.index,
            types,
        }
    }
}

pub type Method = Option<std::ptr::NonNull<objc_method>>;
