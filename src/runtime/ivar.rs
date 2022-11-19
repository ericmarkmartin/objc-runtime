pub struct Ivar {
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

impl Ivar {
    pub fn new(name: String, size: usize, alignment: u8, types: String) -> Self {
        Self {
            name,
            size,
            alignment,
            types,
        }
    }
}
