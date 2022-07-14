use std::num;

use substring::Substring;

use crate::{
    error,
    expr::{self, *},
    token::{DataType, Token, TokenType},
};

pub struct Interpreter {}

impl ExprVisitor for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> VisitorTypes {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> VisitorTypes {
        let left = match expr.left.accept(self) {
            VisitorTypes::DataType(d) => d,
            _ => {
                return self.runtime_error(
                    Some(&expr.operator),
                    "Expected a binary operation with proper data types.",
                );
            }
        };

        let right = match expr.right.accept(self) {
            VisitorTypes::DataType(d) => d,
            _ => {
                return self.runtime_error(
                    Some(&expr.operator),
                    "Expected a binary operation with proper data types.",
                );
            }
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
                _ => {
                    return self.runtime_error(
                        Some(&expr.operator),
                        "Operands must be two numbers or two strings.",
                    );
                }
            },
            TokenType::Minus => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Number(l - r),
                _ => {
                    return self.runtime_error(Some(&expr.operator), "Expected a number.");
                }
            },
            TokenType::Slash => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Number(l / r),
                _ => {
                    return self.runtime_error(Some(&expr.operator), "Expected a number.");
                }
            },
            TokenType::Star => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Number(l * r),
                _ => {
                    return self.runtime_error(Some(&expr.operator), "Expected a number.");
                }
            },
            TokenType::Equalequal => match (left, right) {
                (Some(l), Some(r)) => DataType::Bool(self.is_equal(&l, &r)),
                _ => {
                    return self
                        .runtime_error(Some(&expr.operator), "Expected a binary operation.");
                }
            },
            TokenType::Greater => match (left, right) {
                (Some(DataType::Bool(l)), Some(DataType::Bool(r))) => DataType::Bool(l > r),
                _ => {
                    return self
                        .runtime_error(Some(&expr.operator), "Expected a binary operation.");
                }
            },
            TokenType::Greaterequal => match (left, right) {
                (Some(DataType::Bool(l)), Some(DataType::Bool(r))) => DataType::Bool(l >= r),
                _ => {
                    return self
                        .runtime_error(Some(&expr.operator), "Expected a binary operation.");
                }
            },
            TokenType::Less => match (left, right) {
                (Some(DataType::Bool(l)), Some(DataType::Bool(r))) => DataType::Bool(l < r),
                _ => {
                    return self
                        .runtime_error(Some(&expr.operator), "Expected a binary operation.");
                }
            },
            TokenType::Lessequal => match (left, right) {
                (Some(DataType::Bool(l)), Some(DataType::Bool(r))) => DataType::Bool(l <= r),
                _ => {
                    return self
                        .runtime_error(Some(&expr.operator), "Expected a binary operation.");
                }
            },
            _ => {
                return self.runtime_error(Some(&expr.operator), "Invalid binary operation.");
            }
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
                _ => return self.runtime_error(Some(&expr.operator), "Expected a number."),
            },
            _ => return self.runtime_error(Some(&expr.operator), "Expected a number."),
        };
        match expr.operator.token_type {
            TokenType::Minus => VisitorTypes::DataType(Some(right)),
            TokenType::Bang => {
                VisitorTypes::DataType(Some(DataType::Bool(!self.is_truthy(&right))))
            }
            _ => {
                return self.runtime_error(Some(&expr.operator), "Expected a '!' or '-' operator.")
            }
        }
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> VisitorTypes {
        todo!()
    }
}

impl Interpreter {
    fn interpreter(&mut self, expr: &mut dyn Expr) {
        let value = expr.accept(self);
        match value {
            VisitorTypes::DataType(_) => {
                println!("{}", self.stringify(value));
            }
            VisitorTypes::RunTimeError { token, msg } => match token {
                Some(t) => {
                    error::token_error(&t, msg.as_str());
                }
                None => error::error(0, "Unknown run time error occured."),
            },
            _ => panic!(
                "Unknown visitor type returned.\n This should be an impossible state to be in."
            ),
        }
    }

    fn stringify(&self, visitor_type: VisitorTypes) -> String {
        let result = match visitor_type {
            VisitorTypes::DataType(d) => match d {
                Some(DataType::String(s)) => s,
                Some(DataType::Number(n)) => {
                    let mut number = n.to_string();
                    if number.ends_with(".0") {
                        number = number.substring(0, number.len() - 2).to_string();
                    }
                    number
                }
                Some(DataType::Bool(b)) => {
                    let string = b.to_string();
                    let string = string.substring(0, string.len() - 2);
                    string.to_owned()
                }
                Some(DataType::Nil) => "nil".to_string(),
                None => "nil".to_string(),
            },
            _ => panic!("An error occured during interpretting."),
        };
        result
    }

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

    fn runtime_error(&self, token: Option<&Token>, msg: &str) -> VisitorTypes {
        let token_clone = match token {
            Some(t) => Some(t.clone()),
            None => None,
        };

        VisitorTypes::RunTimeError {
            token: token_clone,
            msg: msg.to_string(),
        }
    }
}
