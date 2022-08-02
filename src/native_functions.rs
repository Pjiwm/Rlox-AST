use std::time::SystemTime;

use crate::{function::LoxCallable, token::DataType};
#[derive(Debug)]
pub struct Clock;
impl LoxCallable for Clock {
    fn call(
        &self,
        _: &crate::interpreter::Interpreter,
        _: Vec<crate::token::DataType>,
    ) -> DataType {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => DataType::Number(n.as_millis() as f64),
            Err(_) => DataType::Nil,
        }
    }

    fn arity(&self) -> usize {
        0
    }
}
