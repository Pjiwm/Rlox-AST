use std::collections::HashMap;

use crate::token::DataType;

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
}