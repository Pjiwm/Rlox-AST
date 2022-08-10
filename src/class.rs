use core::fmt;
use std::fmt::Formatter;

use crate::{function::LoxCallable, interpreter::Interpreter, token::DataType};

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
}
impl LoxClass {
    pub fn new(name: String) -> LoxClass {
        LoxClass { name }
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<DataType>) -> DataType {
        let instance = LoxInstance::new(self.clone());
        DataType::Instance(instance)
    }

    fn arity(&self) -> usize {
        0
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Class {}>", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: LoxClass,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> LoxInstance {
        LoxInstance { class }
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Instance {}>", self.class.name)
    }
}
