use super::selector::objc_selector;

pub struct Property {
    pub(crate) name: String,
    attributes: String,
    type_: String,
    getter: objc_selector,
    setter: objc_selector,
}
