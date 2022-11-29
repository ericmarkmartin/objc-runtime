#[allow(non_camel_case_types)]
pub struct objc_ivar {
    pub(crate) name: String,
    pub(crate) size: usize,
    pub(crate) alignment: u8,
    pub(crate) types: String,
}

enum _Ownership {
    Invalid,
    Strong,
    Weak,
    Unsafe,
}

impl objc_ivar {
    pub fn new(name: String, size: usize, alignment: u8, types: String) -> Self {
        Self {
            name,
            size,
            alignment,
            types,
        }
    }
}

pub type Ivar = Option<std::ptr::NonNull<objc_ivar>>;
