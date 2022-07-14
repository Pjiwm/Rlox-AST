use crate::{
    expr::*,
    token::{DataType, TokenType},
};

pub struct Interpreter {}

impl ExprVisitor for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> VisitorTypes {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> VisitorTypes {
        // TODO panics could be replaced with a better error handling system
        let left = match expr.left.accept(self) {
            VisitorTypes::DataType(d) => d,
            _ => panic!("Expected a number"),
        };

        let right = match expr.right.accept(self) {
            VisitorTypes::DataType(d) => d,
            _ => panic!("Expected a number"),
        };

        let calculation = match expr.operator.token_type {
            // There's extra logic for strings, this is so strings can be concatinated with the + operator.
            TokenType::Plus => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Number(l + r),
                (Some(DataType::String(l)), Some(DataType::String(r))) => {
                    let mut s = String::new();
                    s.push_str(l.as_str());
                    s.push_str(r.as_str());
                    DataType::String(s)
                }
                _ => panic!("Expected a number or string"),
            },
            TokenType::Minus => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Number(l - r),
                _ => panic!("Expected a number"),
            },
            TokenType::Slash => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Number(l / r),
                _ => panic!("Expected a number"),
            },
            TokenType::Star => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Number(l * r),
                _ => panic!("Expected a number"),
            },
            TokenType::Equalequal => match (left, right) {
                (Some(l), Some(r)) => DataType::Bool(self.is_equal(&l, &r)),
                _ => panic!("Expected a binary operation"),
            },
            _ => panic!("No such binary operator"),
        };
        VisitorTypes::DataType(Some(calculation))
    }

    fn visit_call_expr(&mut self, expr: &Call) -> VisitorTypes {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &Get) -> VisitorTypes {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> VisitorTypes {
        expr.expression.accept(self)
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> VisitorTypes {
        VisitorTypes::DataType(expr.value.clone())
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> VisitorTypes {
        todo!()
    }

    fn visit_set_expr(&mut self, expr: &Set) -> VisitorTypes {
        todo!()
    }

    fn visit_super_expr(&mut self, expr: &Super) -> VisitorTypes {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &This) -> VisitorTypes {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> VisitorTypes {
        // So right now if we have a none number it just becomes 0.0...
        let right = match expr.right.accept(self) {
            VisitorTypes::DataType(d) => match d.unwrap() {
                DataType::Number(n) => DataType::Number(-n),
                _ => DataType::Number(0.0),
            },
            _ => DataType::Number(0.0),
        };
        match expr.operator.token_type {
            TokenType::Minus => VisitorTypes::DataType(Some(right)),
            TokenType::Bang => VisitorTypes::DataType(Some(DataType::Bool(!self.is_truthy(&right)))),
            _ => todo!(),
        }
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> VisitorTypes {
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

    fn is_equal(&self, a: &DataType, b: &DataType) -> bool {
        match (a, b) {
            (DataType::Number(a), DataType::Number(b)) => a == b,
            (DataType::String(a), DataType::String(b)) => a == b,
            (DataType::Bool(a), DataType::Bool(b)) => a == b,
            (DataType::Nil, DataType::Nil) => true,
            _ => false,
        }
    }
}
