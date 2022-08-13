use crate::{
    ast::{Function, Stmt, VisitorTypes},
    class::LoxInstance,
    environment::Environment,
    interpreter::Interpreter,
    token::{DataType, Token},
};
use std::{
    cell::RefCell,
    fmt::{self, Debug, Display, Formatter},
    rc::Rc,
};

pub trait LoxCallable: Debug + Display {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<DataType>) -> DataType;
    fn arity(&self) -> usize;
}

#[derive(Clone)]
pub struct LoxFunction {
    pub body: Rc<Vec<Rc<dyn Stmt>>>,
    pub params: Rc<Vec<Token>>,
    name: Box<Token>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: &Function, closure: &Rc<RefCell<Environment>>) -> LoxFunction {
        LoxFunction {
            body: Rc::clone(&declaration.body),
            params: Rc::clone(&declaration.params),
            name: Box::new(declaration.name.dup()),
            closure: Rc::clone(closure),
        }
    }

    pub fn bind(&self, instance: Rc<LoxInstance>) -> LoxFunction {
        let env = RefCell::new(Environment::new_enclosing(Rc::clone(&self.closure)));
        env.borrow_mut().define("this".to_string(), DataType::Instance(instance.clone()));
        LoxFunction {
            body: Rc::clone(&self.body),
            params: Rc::clone(&self.params),
            name: self.name.clone(),
            closure: Rc::new(env),
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<DataType>) -> DataType {
        let mut environment = Environment::new_enclosing(Rc::clone(&self.closure));
        for (i, token) in self.params.iter().enumerate() {
            let value = match arguments.get(i) {
                Some(d) => d.clone(),
                None => DataType::Nil,
            };
            environment.define(token.dup().lexeme, value);
        }
        let statements = Rc::new(&self.body);
        match interpreter.execute_block(&statements, environment) {
            VisitorTypes::Return(Some(d)) => d,
            _ => DataType::Nil,
        }
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
impl fmt::Display for LoxNative {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.function)
    }
}
