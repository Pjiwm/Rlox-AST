use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use crate::{
    ast::VisitorTypes,
    token::{DataType, Token},
};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, DataType>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosing(enclosing: Rc<RefCell<Environment>>) -> Environment {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: DataType) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> VisitorTypes {
        if let Some(object) = self.values.get(&name.dup().lexeme) {
            return VisitorTypes::DataType(Some(object.clone()));
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().get(name)
        } else {
            VisitorTypes::RunTimeError {
                token: Some(name.dup()),
                msg: format!("Variable {} is not defined.", name.lexeme),
            }
        }
    }

    pub fn get_at(&self, distance: usize, name: &str) -> VisitorTypes {
        if distance == 0 {
            VisitorTypes::DataType(Some(self.values.get(&name.to_string()).unwrap().clone()))
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow()
                .get_at(distance - 1, name)
        }
    }

    pub fn assign(&mut self, name: &Token, value: DataType) -> VisitorTypes {
        if let Entry::Occupied(mut object) = self.values.entry(name.dup().lexeme) {
            object.insert(value);
            return VisitorTypes::Void(());
        } else if let Some(ref mut enclosing) = self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }
        VisitorTypes::RunTimeError {
            token: Some(name.dup()),
            msg: format!("Variable {} is not defined.", name.lexeme),
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: DataType) -> VisitorTypes {
        if distance == 0 {
            self.values.insert(name.dup().lexeme, value);
            return VisitorTypes::Void(());
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign_at(distance - 1, name, value)
        }
    }
}
