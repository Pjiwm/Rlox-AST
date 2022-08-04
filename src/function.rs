use crate::{
    ast::{Function, Stmt},
    environment::{self, Environment},
    interpreter::Interpreter,
    token::{DataType, Token},
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
    body: Rc<Vec<Rc<dyn Stmt>>>,
    params: Rc<Vec<Token>>,
    name: Box<Token>
}

impl LoxFunction {
    pub fn new(declaration: &Function) -> LoxFunction {
        LoxFunction {
            body: Rc::clone(&declaration.body),
            params: Rc::clone(&declaration.params),
            name: Box::new(declaration.name.dup())
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<DataType>) -> DataType {
        let mut environment = Environment::new_enclosing(Rc::clone(&interpreter.globals));
        for (i, token) in self.params.iter().enumerate() {
            let value = match arguments.get(i) {
                Some(d) => d.clone(),
                None => DataType::Nil,
            };
            environment.define(token.dup().lexeme, value);
        }
        let statements = Rc::new(&self.body);
        interpreter.execute_block(&statements, environment);
        DataType::Nil
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Function {}>", self.name.lexeme)
    }
}

impl Debug for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("<Function {}>", self.name.lexeme);
        f.debug_struct("LoxFunction")
            .field("name:", &value)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct LoxNative {
    pub function: Rc<dyn LoxCallable>,
}
