use crate::{
    ast::Function,
    environment::{self, Environment},
    interpreter::Interpreter,
    token::DataType,
};
use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{self, Debug, Display, Formatter, Pointer},
    rc::Rc,
};

pub trait LoxCallable: Debug + Display {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<DataType>) -> DataType;
    fn arity(&self) -> usize;
}

#[derive(Clone)]
pub struct LoxFunction {
    declaration: Rc<Function>,
}

impl LoxFunction {
    fn new(declaration: Rc<Function>) -> LoxFunction {
        LoxFunction { declaration }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<DataType>) -> DataType {
        let mut environment = Environment::new_enclosing(Rc::clone(&interpreter.globals));
        for (i, token) in self.declaration.params.iter().enumerate() {
            let value = match arguments.get(i) {
                Some(d) => d.clone(),
                None => DataType::Nil,
            };
            environment.define(token.dup().lexeme, value);
        }
        let statements = Rc::new(&self.declaration.body);
        interpreter.execute_block(&statements, environment);
        DataType::Nil
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Function {}>", self.declaration.name.lexeme)
    }
}

impl Debug for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("<Function {}>", self.declaration.name.lexeme);
        f.debug_struct("LoxFunction")
            .field("name:", &value)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct LoxNative {
    pub function: Rc<dyn LoxCallable>,
}
