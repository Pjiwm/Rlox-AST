use std::{collections::HashMap, io::Error};

use crate::{
    ast::VisitorTypes,
    token::{DataType, Token},
};

pub struct Environment {
    pub values: HashMap<String, DataType>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: DataType) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<&DataType> {
        self.values.get(name)
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
