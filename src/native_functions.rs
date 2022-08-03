use std::{
    fmt::{self, Display, Formatter},
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
