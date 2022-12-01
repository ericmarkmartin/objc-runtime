use pow_of_2::PowOf2;

#[allow(non_camel_case_types)]
pub struct objc_ivar {
    pub(crate) name: String,
    pub(crate) size: usize,
    pub(crate) alignment: PowOf2<usize>,
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
            alignment: PowOf2::from_exp(alignment),
            types,
        }
    }
}

pub type Ivar = Option<std::ptr::NonNull<objc_ivar>>;
