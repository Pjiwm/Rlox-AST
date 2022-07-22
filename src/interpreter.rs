use std::{
    borrow::Borrow,
    io::{self, Error, ErrorKind},
};

use substring::Substring;

use crate::{
    ast::*,
    environment::{self, Environment},
    error,
    token::{DataType, Token, TokenType},
};
pub struct Interpreter {
    environment: Environment,
}
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Box<dyn Stmt>>) {
        for stmt in statements {
            self.execute(&stmt);
        }
    }

    fn execute(&mut self, stmt: &Box<dyn Stmt>) {
        stmt.accept(self);
    }

    fn execute_block(&mut self, statements: &Box<Vec<Box<dyn Stmt>>>, environment: Environment) {
        let previous = environment;
        for stmt in statements.iter() {
            self.execute(stmt.clone());
        }
        self.environment = previous;
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

    fn concatinate(&self, l: &str, r: &str) -> DataType {
        let mut s = String::new();
        s.push_str(l);
        s.push_str(r);
        DataType::String(s)
    }

    fn runtime_error(&self, token: &Option<Token>, message: &str) -> Error {
        error::runtime_error(token, message);
        io::Error::new(ErrorKind::Other, message)
    }
}

impl ExprVisitor for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> VisitorTypes {
        let value = expr.value.accept(self);
        match value {
            VisitorTypes::DataType(d) => {
                let data_type_value = match d {
                    Some(d) => d,
                    None => panic!("Interpreter entered an impossible state."),
                };
                self.environment.assign(expr.name.dup(), data_type_value)
            }
            _ => VisitorTypes::RunTimeError {
                token: Some(expr.name.dup()),
                msg: "Invalid assignment target.".to_string(),
            },
        }
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
                    self.concatinate(l.as_str(), r.as_str())
                }
                // This makes it possible to concatinate a number with a string.
                // Doing this is easier for the user when they want to print numbers and strings together.
                (Some(DataType::Number(l)), Some(DataType::String(r))) => {
                    self.concatinate(l.to_string().as_str(), r.as_str())
                }
                (Some(DataType::String(l)), Some(DataType::Number(r))) => {
                    self.concatinate(l.as_str(), r.to_string().as_str())
                }
                // Concatinating a string with a bool
                (Some(DataType::String(l)), Some(DataType::Bool(r))) => {
                    self.concatinate(l.as_str(), r.to_string().as_str())
                }
                (Some(DataType::Bool(l)), Some(DataType::String(r))) => {
                    self.concatinate(l.to_string().as_str(), r.as_str())
                }
                // Concatinating a bool with a number
                (Some(DataType::Bool(l)), Some(DataType::Number(r))) => {
                    self.concatinate(l.to_string().as_str(), r.to_string().as_str())
                }
                (Some(DataType::Number(l)), Some(DataType::Bool(r))) => {
                    self.concatinate(l.to_string().as_str(), r.to_string().as_str())
                }
                // Concatinating Nil with a string
                (None, Some(DataType::String(r))) => self.concatinate("nil", r.as_str()),
                (Some(DataType::String(l)), None) => self.concatinate(l.as_str(), "nil"),
                // Concatinating Nil with a number
                (None, Some(DataType::Number(r))) => {
                    self.concatinate("nil", r.to_string().as_str())
                }
                (Some(DataType::Number(l)), None) => {
                    self.concatinate(l.to_string().as_str(), "nil")
                }
                // Concatinating Nil with a bool
                (None, Some(DataType::Bool(r))) => self.concatinate("nil", r.to_string().as_str()),
                (Some(DataType::Bool(l)), None) => self.concatinate(l.to_string().as_str(), "nil"),
                _ => {
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Operands must be a number or string.",
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
        self.environment.get(expr.name.dup())
    }
}

impl StmtVisitor for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &Block) -> VisitorTypes {
        self.execute_block(
            &stmt.statements,
            Environment::new_enclosing(Box::new(self.environment.borrow().clone())),
        );
        VisitorTypes::Void(())
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
            Err(_) => {}
        }
        VisitorTypes::Void(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> VisitorTypes {
        todo!()
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> VisitorTypes {
        let mut data_type = None;
        if let Some(initializer) = &stmt.initializer {
            data_type = match Some(initializer.accept(self)) {
                Some(v) => match v {
                    VisitorTypes::DataType(d) => d,
                    _ => return self.visitor_runtime_error(Some(&stmt.name), "Expected a value."),
                },
                None => return self.visitor_runtime_error(Some(&stmt.name), "Expected a value."),
            }
        }
        let value = match data_type {
            Some(v) => v,
            None => DataType::Nil,
        };
        self.environment.define(stmt.name.lexeme.clone(), value);
        VisitorTypes::Void(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> VisitorTypes {
        todo!()
    }
}
