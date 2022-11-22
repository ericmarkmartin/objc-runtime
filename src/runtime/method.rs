use super::{object::Object, selector::Selector};

pub type Imp = fn(&Object, &Selector, Vec<&Object>);

pub struct Method<'a> {
    imp: &'a Imp,
    selector: &'a Selector,
    types: String,
}

impl<'a> Method<'a> {
    pub fn new(imp: &'a Imp, selector: &'a Selector, types: String) -> Self {
        Self {
            imp,
            selector,
            types,
        }
    }
}
