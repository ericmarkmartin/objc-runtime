use super::class::Class;
use std::rc::Weak;

pub struct Object {
    /// Pointer to this object's class.
    isa: Weak<Class>,
}
