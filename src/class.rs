use core::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
}
impl LoxClass {
    pub fn new(name: String) -> LoxClass {
        LoxClass { name }
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Class {}>", self.name)
    }
}
