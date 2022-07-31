use crate::{interpreter::Interpreter, token::DataType};

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<DataType>) -> DataType;
    fn arity(&self) -> usize;
}
// TODO polish LoxFunction struct, as we only created it so far to work on the interpreter.
#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub arity: usize
}
impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<DataType>) -> DataType {
        DataType::Nil
    }

    fn arity(&self) -> usize {
        self.arity
    }
}
