use core::fmt;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Formatter,
    rc::Rc,
};

use crate::{
    ast::VisitorTypes,
    function::{LoxCallable, LoxFunction},
    interpreter::Interpreter,
    token::{DataType, Token},
};

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, LoxFunction>,
}
impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, LoxFunction>) -> LoxClass {
        LoxClass { name, methods }
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, _: &mut Interpreter, _: Vec<DataType>) -> DataType {
        let instance = LoxInstance::new(self.clone());
        DataType::Instance(Rc::new(instance))
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
            return VisitorTypes::DataType(Some(
                self.fields.borrow().get(&token.lexeme).unwrap().clone(),
            ));
        }
        // Might put this in a seperate function later.
        if self.class.methods.contains_key(&token.lexeme) {
            let method = self.class.methods.get(&token.lexeme).unwrap().clone();
            // method.bind(Rc::new(self.clone()));
            return VisitorTypes::DataType(Some(DataType::Function(method.bind(Rc::new(self.clone())))));
        }

        VisitorTypes::RunTimeError {
            token: Some(token.dup()),
            msg: format!("Undefined property '{}'.", token.lexeme),
        }
    }

    pub fn set(&self, token: &Token, value: Option<DataType>) {
        self.fields
            .borrow_mut()
            .insert(token.dup().lexeme, value.unwrap_or(DataType::Nil));
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Instance {}>", self.class.name)
    }
}
