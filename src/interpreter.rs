use std::io::{self, Error, ErrorKind};

use substring::Substring;

use crate::{
    error,
    expr::*,
    token::{DataType, Token, TokenType},
};
pub struct Interpreter;
impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&mut self, statements: Vec<Box<dyn Stmt>>) {
        for stmt in statements {
            self.execute(stmt);
        }
    }

    fn execute(&mut self, stmt: Box<dyn Stmt>) {
        stmt.accept(self);
    }

    fn stringify(&self, visitor_type: VisitorTypes) -> Result<String, Error> {
        match visitor_type {
            VisitorTypes::DataType(d) => Ok(self.stringify_helper(d)),
            VisitorTypes::RunTimeError { token, msg } => {
                Err(self.runtime_error(&token, msg.as_str()))
            }
            _ => panic!("Interpreter entered an impossible state."),
        }
    }

    fn stringify_helper(&self, data_type: Option<DataType>) -> String {
        let result = match data_type {
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
                    string.to_owned()
                }
                Some(DataType::Nil) => "nil".to_string(),
                None => "nil".to_string(),
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

    fn visitor_runtime_error(&self, token: Option<&Token>, msg: &str) -> VisitorTypes {
        let token_clone = match token {
            Some(t) => Some(t.clone()),
            None => None,
        };

        VisitorTypes::RunTimeError {
            token: token_clone,
            msg: msg.to_string(),
        }
    }

    fn runtime_error(&self, token: &Option<Token>, message: &str) -> Error {
        error::runtime_error(token, message);
        io::Error::new(ErrorKind::Other, message)
    }
}

impl ExprVisitor for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> VisitorTypes {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> VisitorTypes {
        let left = match expr.left.accept(self) {
            VisitorTypes::DataType(d) => d,
            _ => {
                return self.visitor_runtime_error(
                    Some(&expr.operator),
                    "Expected a binary operation with proper data types.",
                );
            }
        };

        let right = match expr.right.accept(self) {
            VisitorTypes::DataType(d) => d,
            _ => {
                return self.visitor_runtime_error(
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
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Operands must be two numbers or two strings.",
                    );
                }
            },
            TokenType::Minus => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Number(l - r),
                _ => {
                    return self.visitor_runtime_error(Some(&expr.operator), "Expected a number.");
                }
            },
            TokenType::Slash => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Number(l / r),
                _ => {
                    return self.visitor_runtime_error(Some(&expr.operator), "Expected a number.");
                }
            },
            TokenType::Star => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Number(l * r),
                _ => {
                    return self.visitor_runtime_error(Some(&expr.operator), "Expected a number.");
                }
            },
            TokenType::Equalequal => match (left, right) {
                (Some(l), Some(r)) => DataType::Bool(self.is_equal(&l, &r)),
                (None, None) => DataType::Bool(true),
                (Some(_), None) => DataType::Bool(false),
                (None, Some(_)) => DataType::Bool(false),
                _ => {
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Expected a binary operation.",
                    );
                }
            },
            TokenType::Greater => match (left, right) {
                (Some(DataType::Bool(l)), Some(DataType::Bool(r))) => DataType::Bool(l > r),
                _ => {
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Expected a binary operation.",
                    );
                }
            },
            TokenType::Greaterequal => match (left, right) {
                (Some(DataType::Bool(l)), Some(DataType::Bool(r))) => DataType::Bool(l >= r),
                _ => {
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Expected a binary operation.",
                    );
                }
            },
            TokenType::Less => match (left, right) {
                (Some(DataType::Bool(l)), Some(DataType::Bool(r))) => DataType::Bool(l < r),
                _ => {
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Expected a binary operation.",
                    );
                }
            },
            TokenType::Lessequal => match (left, right) {
                (Some(DataType::Bool(l)), Some(DataType::Bool(r))) => DataType::Bool(l <= r),
                _ => {
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Expected a binary operation.",
                    );
                }
            },
            _ => {
                return self
                    .visitor_runtime_error(Some(&expr.operator), "Invalid binary operation.");
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
        // The book uses an evaluate function to execute this line of code.
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
            VisitorTypes::DataType(d) => match d {
                Some(v) => match v {
                    DataType::Number(v) => DataType::Number(-v),
                    _ => {
                        return self
                            .visitor_runtime_error(Some(&expr.operator), "Expected a number.")
                    }
                },
                None => DataType::Nil,
            },
            _ => return self.visitor_runtime_error(Some(&expr.operator), "Expected a number."),
        };
        match expr.operator.token_type {
            TokenType::Minus => VisitorTypes::DataType(Some(right)),
            TokenType::Bang => {
                VisitorTypes::DataType(Some(DataType::Bool(!self.is_truthy(&right))))
            }
            _ => {
                return self
                    .visitor_runtime_error(Some(&expr.operator), "Expected a '!' or '-' operator.")
            }
        }
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> VisitorTypes {
        todo!()
    }
}

impl StmtVisitor for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &Block) -> VisitorTypes {
        todo!()
    }

    fn visit_class_stmt(&mut self, stmt: &Class) -> VisitorTypes {
        todo!()
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> VisitorTypes {
        stmt.expression.accept(self);
        VisitorTypes::Void(())
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> VisitorTypes {
        todo!()
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> VisitorTypes {
        todo!()
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> VisitorTypes {
        let value = stmt.expression.accept(self);
        match self.stringify(value) {
            Ok(s) => println!("{}", s),	
            Err(_) => {},
        }
        VisitorTypes::Void(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> VisitorTypes {
        todo!()
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> VisitorTypes {
        todo!()
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> VisitorTypes {
        todo!()
    }
}
