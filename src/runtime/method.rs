use super::{object::Object, selector::Selector};

pub type Imp = fn(&Object, &Selector, Vec<&Object>);

pub struct Method {
    imp: Imp,
    selector: Selector,
    types: String,
}
