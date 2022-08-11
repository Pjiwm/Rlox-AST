use core::fmt;
use std::{cell::RefCell, collections::HashMap, fmt::Formatter};

use crate::{
    ast::VisitorTypes,
    function::LoxCallable,
    interpreter::Interpreter,
    token::{DataType, Token},
};

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
    fn call(&self, _: &mut Interpreter, _: Vec<DataType>) -> DataType {
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
    fields: RefCell<HashMap<String, DataType>>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> LoxInstance {
        LoxInstance {
            class,
            fields: RefCell::new(HashMap::new()),
        }
    }
    pub fn get(&self, token: &Token) -> VisitorTypes {
        if self.fields.borrow().contains_key(&token.lexeme) {
            return VisitorTypes::Return(Some(
                self.fields.borrow().get(&token.lexeme).unwrap().clone(),
            ));
        }
        VisitorTypes::RunTimeError {
            token: Some(token.dup()),
            msg: format!("Undefined property '{}'.", token.lexeme),
        }
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Instance {}>", self.class.name)
    }
}
