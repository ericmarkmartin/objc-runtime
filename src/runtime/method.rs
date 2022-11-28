use super::{object::Object, selector::Selector};

pub type Imp = fn(&Object, &Selector, Vec<&Object>);

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
