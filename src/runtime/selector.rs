pub enum Registration {
    /// The name of this selector, if unregistered
    Name(String),
    /// The index of this selector in the selector table. When a selector is
    /// registered with the runtime, its name is replaced by an indedx uniquely
    /// identifying it. The index is used for dispatch.
    Index(usize),
}

pub struct Selector {
    registration: Registration,
    types: String,
}
