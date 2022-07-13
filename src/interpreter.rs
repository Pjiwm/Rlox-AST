use crate::{
    expr::{self, *},
    token::{DataType, TokenType},
};

pub struct Interpreter {}

impl ExprVisitor for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> ReturnTypes {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> ReturnTypes {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &Call) -> ReturnTypes {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &Get) -> ReturnTypes {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> ReturnTypes {
        expr.expression.accept(self)
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> ReturnTypes {
        ReturnTypes::DataType(expr.value.clone())
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> ReturnTypes {
        todo!()
    }

    fn visit_set_expr(&mut self, expr: &Set) -> ReturnTypes {
        todo!()
    }

    fn visit_super_expr(&mut self, expr: &Super) -> ReturnTypes {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &This) -> ReturnTypes {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> ReturnTypes {
        // So right now if we have a none number it just becomes 0.0...
        let right = match expr.right.accept(self) {
            ReturnTypes::DataType(d) => match d.unwrap() {
                DataType::Number(n) => DataType::Number(-n),
                _ => DataType::Number(0.0),
            },
            _ => DataType::Number(0.0),
        };
        match expr.operator.token_type {
            TokenType::Minus => ReturnTypes::DataType(Some(right)),
            // TODO fix this tomorrow
            TokenType::Bang => ReturnTypes::DataType(Some(DataType::Bool(!self.is_truthy(&right)))),
            _ => todo!(),
        }
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> ReturnTypes {
        todo!()
    }
}

impl Interpreter {
    fn is_truthy(&self, data_type: &DataType) -> bool {
        match data_type {
            DataType::Bool(b) => *b,
            DataType::Nil => false,
            _ => true,
        }
    }
}
