use super::context::SelectorKey;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct SelectorInfo {
    pub(crate) name: String,
    pub(crate) types: Option<String>,
}

pub struct Selector {
    pub(crate) index: SelectorKey,
    pub(crate) selector_info: SelectorInfo,
}

impl SelectorInfo {
    pub(crate) fn new(name: String) -> Self {
        Self { name, types: None }
    }
}
