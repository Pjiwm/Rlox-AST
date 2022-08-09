use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
    time::SystemTime,
};

use crate::{function::LoxCallable, interpreter::Interpreter, token::DataType};
#[derive(Debug)]
pub struct Clock {
    name: String,
}
impl Clock {
    pub fn new(name: String) -> Clock {
        Clock { name }
    }
}
impl LoxCallable for Clock {
    fn call(&self, _: &mut Interpreter, _: Vec<crate::token::DataType>) -> DataType {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => DataType::Number(n.as_millis() as f64),
            Err(_) => DataType::Nil,
        }
    }

    fn arity(&self) -> usize {
        0
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Native-Function {}>", self.name)
    }
}
#[derive(Debug)]
pub struct Println {
    name: String,
}
impl Println {
    pub fn new(name: String) -> Println {
        Println { name }
    }
}

impl LoxCallable for Println {
    fn call(&self, _: &mut Interpreter, arguments: Vec<DataType>) -> DataType {
        println!("{}", arguments[0].to_string());
        DataType::Nil
    }

    fn arity(&self) -> usize {
        1
    }
}

impl Display for Println {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Native-Function {}>", self.name)
    }
}
