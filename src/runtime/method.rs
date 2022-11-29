use super::{
    message::id,
    selector::{Selector, SEL},
};

pub type Imp = extern "C" fn(id, SEL, ...);

pub struct Method<'a> {
    pub(crate) imp: &'a mut Imp,
    pub(crate) selector: &'a Selector,
    types: String,
}

impl<'a> Method<'a> {
    pub fn new(imp: &'a mut Imp, selector: &'a Selector, types: String) -> Self {
        Self {
            imp,
            selector,
            types,
        }
    }
}
