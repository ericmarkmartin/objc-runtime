use super::selector::Selector;

pub struct Property {
    pub(crate) name: String,
    attributes: String,
    type_: String,
    getter: Selector,
    setter: Selector,
}
