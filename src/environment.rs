use std::{collections::HashMap};

use crate::{
    ast::VisitorTypes,
    token::{DataType, Token},
};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, DataType>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosing(enclosing: Box<Environment>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: DataType) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> VisitorTypes {
        if self.values.contains_key(&name.lexeme) {
            let value = Some(self.values[&name.lexeme].dup());
            return VisitorTypes::DataType(value);
        }
        if let Some(ref env) = self.enclosing {
            return env.get(name);
        }
        VisitorTypes::RunTimeError {
            token: Some(name.dup()),
            msg: format!("Variable {} is not defined.", name.lexeme),
        }
    }

    pub fn assign(&mut self, name: Token, value: DataType) -> VisitorTypes {
        if self.values.contains_key(name.lexeme.as_str()) {
            let v = self.values.insert(name.lexeme.clone(), value.clone());
            VisitorTypes::DataType(v)
        } else {
            VisitorTypes::RunTimeError {
                msg: "Undefined variable".to_string(),
                token: Some(name.dup()),
            }
        }
    }
}
