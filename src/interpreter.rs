use std::{
    cell::RefCell,
    io::{self, Error, ErrorKind},
    rc::Rc, ops::DerefMut,
};

use colored::Colorize;
use lazy_static::__Deref;
use substring::Substring;

use crate::{
    ast::*,
    environment::Environment,
    error,
    function::{LoxCallable, LoxFunction, LoxNative},
    native_functions::Clock,
    token::{self, DataType, Token, TokenType},
};
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: RefCell<Rc<RefCell<Environment>>>,
    is_repl: bool,
    is_last_statement: bool,
}
impl Interpreter {
    pub fn new(is_repl: bool) -> Interpreter {
        let globals = Rc::new(RefCell::new(Environment::new()));
        let clock = DataType::Native(LoxNative {
            function: Rc::new(Clock::new("Clock".to_string())),
        });
        globals.borrow_mut().define("clock".to_string(), clock);

        Interpreter {
            globals: Rc::clone(&globals),
            environment: RefCell::new(Rc::clone(&globals)),
            is_repl,
            is_last_statement: false,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Rc<dyn Stmt>>) {
        for (i, stmt) in statements.iter().enumerate() {
            self.is_last_statement = i == statements.len() - 1;
            self.execute(&stmt);
        }
    }

    pub fn execute_block(
        &mut self,
        statements: &Rc<Vec<Rc<dyn Stmt>>>,
        environment: Environment,
    ) {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));
        for stmt in statements.iter() {
            self.execute(&stmt);
        }
        self.environment.replace(previous);
    }

    fn execute(&mut self, stmt: &Rc<dyn Stmt>) {
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
            Some(_) => "Function".to_string(),
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

    fn repl_printer(&self, expr: &VisitorTypes) {
        match expr {
            VisitorTypes::DataType(d) => {
                let value = self.repl_stringify(d.clone());
                println!("{value}");
            }
            _ => {}
        }
    }

    fn repl_stringify(&self, data_type: Option<DataType>) -> String {
        let result = match data_type {
            Some(DataType::String(s)) => s.yellow().to_string(),
            Some(DataType::Number(n)) => {
                let mut number = n.to_string();
                if number.ends_with(".0") {
                    number = number.substring(0, number.len() - 2).to_string();
                }
                number.blue().to_string()
            }
            Some(DataType::Bool(b)) => {
                let string = b.to_string().green();
                string.to_string()
            }
            Some(DataType::Nil) => "nil".red().to_string(),
            Some(DataType::Function(f)) => format!("{}", f).on_white().black().to_string(),
            Some(DataType::Native(n)) => format!("{}", n.function)
                .on_white()
                .black()
                .bold()
                .to_string(),
            None => "nil".red().to_string(),
        };
        result
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
                self.environment
                    .borrow()
                    .borrow_mut()
                    .assign(&expr.name, data_type_value.clone())
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
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => {
                    return VisitorTypes::DataType(Some(DataType::Number(l + r)));
                }
                (None, None) => self.concatinate("nil", "nil"),
                (None, Some(r)) => self.concatinate("nil", &r.to_string()),
                (Some(l), None) => self.concatinate(&l.to_string(), "nil"),
                (Some(l), Some(r)) => self.concatinate(&l.to_string(), &r.to_string()),
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
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Bool(l > r),
                _ => {
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Expected a binary operation.",
                    );
                }
            },
            TokenType::Greaterequal => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Bool(l >= r),
                _ => {
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Expected a binary operation.",
                    );
                }
            },
            TokenType::Less => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Bool(l < r),
                _ => {
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Expected a binary operation.",
                    );
                }
            },
            TokenType::Lessequal => match (left, right) {
                (Some(DataType::Number(l)), Some(DataType::Number(r))) => DataType::Bool(l <= r),
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
        let token = expr.paren.dup();
        let callee = match expr.callee.accept(self) {
            VisitorTypes::DataType(d) => d,
            VisitorTypes::RunTimeError { token, msg } => {
                return VisitorTypes::RunTimeError { token, msg }
            }
            _ => panic!("Interpreter entered impossible state."),
        };
        let mut arguments = Vec::<DataType>::new();
        for expr in &expr.arguments {
            let data_type = match expr.accept(self) {
                VisitorTypes::DataType(s) => s,
                _ => panic!("Interpreter entered impossible state."),
            };
            if let Some(d) = data_type {
                arguments.push(d);
            }
            // LoxCallable function = (LoxCallable)callee;
        }
        let function: Rc<dyn LoxCallable>;
        if let Some(c) = callee {
            function = match c {
                DataType::Function(f) => Rc::new(f),
                DataType::Native(n) => n.function,
                _ => {
                    return VisitorTypes::RunTimeError {
                        token: Some(token),
                        msg: "Can only call functions and classes.".to_string(),
                    };
                }
            }
        } else {
            return VisitorTypes::RunTimeError {
                token: Some(token),
                msg: "Can only call functions and classes.".to_string(),
            };
        }

        if arguments.len() != function.arity() {
            return VisitorTypes::RunTimeError {
                token: Some(token),
                msg: format!(
                    "Expected {} arguments but got {}.",
                    function.arity(),
                    arguments.len()
                ),
            };
        }

        VisitorTypes::DataType(Some(function.call(self, arguments)))
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
        let left = match expr.left.accept(self) {
            VisitorTypes::DataType(d) => match d {
                Some(d) => d,
                None => {
                    return self.visitor_runtime_error(
                        Some(&expr.operator),
                        "Expected a binary operation with proper data types.",
                    );
                }
            },
            _ => {
                return self.visitor_runtime_error(
                    Some(&expr.operator),
                    "Expected a binary operation with proper data types.",
                );
            }
        };
        if expr.operator.token_type == TokenType::Or {
            if self.is_truthy(&left) {
                return VisitorTypes::DataType(Some(left));
            }
        } else {
            if !self.is_truthy(&left) {
                return VisitorTypes::DataType(Some(left));
            }
        }
        expr.right.accept(self)
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
        self.environment.borrow().borrow_mut().get(&expr.name)
    }
}

impl StmtVisitor for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &Block) -> VisitorTypes {
        let env = Environment::new_enclosing(self.environment.borrow().clone());
        self.execute_block(&stmt.statements, env);
        VisitorTypes::Void(())
    }

    fn visit_class_stmt(&mut self, stmt: &Class) -> VisitorTypes {
        todo!()
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> VisitorTypes {
        let expr = stmt.expression.accept(self);
        if self.is_repl && self.is_last_statement {
            self.repl_printer(&expr);
        }
        VisitorTypes::Void(())
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> VisitorTypes {
        let function = LoxFunction::new(stmt);
        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.dup().lexeme, DataType::Function(function));
        VisitorTypes::Void(())
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> VisitorTypes {
        let condition = match stmt.condition.accept(self) {
            VisitorTypes::DataType(d) => match d {
                Some(s) => s,
                None => return self.visitor_runtime_error(None, "Expected a condition."),
            },
            _ => return self.visitor_runtime_error(None, "Expected a condition."),
        };
        if self.is_truthy(&condition) {
            self.execute(&stmt.then_branch);
        } else if stmt.else_branch.is_some() {
            self.execute(&stmt.else_branch.as_ref().unwrap());
        }
        VisitorTypes::Void(())
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
        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.dup().lexeme, value);
        VisitorTypes::Void(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> VisitorTypes {
        let mut condition_valid = true;
        while condition_valid {
            let condition = match stmt.condition.accept(self) {
                VisitorTypes::DataType(d) => match d {
                    Some(s) => s,
                    None => return self.visitor_runtime_error(None, "Expected a condition."),
                },
                _ => return self.visitor_runtime_error(None, "Expected a condition."),
            };
            if self.is_truthy(&condition) {
                self.execute(&stmt.body);
            } else {
                condition_valid = false;
            }
        }
        VisitorTypes::Void(())
    }
}
