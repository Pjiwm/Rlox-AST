use std::rc::Rc;

use crate::{ast::*, token::DataType};

pub struct AstPrinter;
impl AstPrinter {
    pub fn _new() -> Self {
        Self
    }

    pub fn _print(&mut self, expr: Rc<dyn Expr>) -> String {
        match expr.accept(self) {
            VisitorTypes::String(s) => s,
            VisitorTypes::DataType(_) => "Incorrect expression".to_string(),
            VisitorTypes::Return(_) => "Incorrect expression".to_string(),
            VisitorTypes::RunTimeError { .. } => {
                "Ran into Run time error: Incorrect expression".to_string()
            }
            VisitorTypes::Void(_) => "Void".to_string(),
        }
    }

    fn parenthesize(&mut self, name: &str, exprs: Vec<&dyn Expr>) -> VisitorTypes {
        let mut s = String::new();
        s.push('(');
        s.push_str(name);
        for expr in exprs {
            s.push(' ');
            let expr_str = match expr.accept(self) {
                VisitorTypes::String(s) => s,
                VisitorTypes::DataType(_) => "Incorrect expression".to_string(),
                VisitorTypes::Return(_) => "Incorrect expression".to_string(),
                VisitorTypes::RunTimeError { .. } => {
                    "Ran into Run time error: Incorrect expression".to_string()
                }
                VisitorTypes::Void(_) => "Void".to_string(),
            };
            s.push_str(expr_str.as_str());
        }
        s.push(')');
        VisitorTypes::String(s)
    }
}

impl ExprVisitor for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> VisitorTypes {
        let expressions = vec![expr.left.as_ref(), expr.right.as_ref()];
        self.parenthesize(&expr.operator.lexeme, expressions)
    }

    fn visit_call_expr(&mut self, expr: &Call) -> VisitorTypes {
        let mut expressions = vec![expr.callee.as_ref()];
        for arg in &expr.arguments {
            expressions.push(arg.as_ref());
        }
        self.parenthesize(&expr.paren.lexeme, expressions)
    }

    fn visit_get_expr(&mut self, expr: &Get) -> VisitorTypes {
        let expressions = vec![expr.object.as_ref()];
        self.parenthesize(&expr.name.lexeme, expressions)
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> VisitorTypes {
        let expressions = vec![expr.expression.as_ref()];
        self.parenthesize("group", expressions)
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> VisitorTypes {
        if expr.value.is_none() {
            VisitorTypes::String("nil".to_owned())
        } else {
            match expr.value.as_ref().unwrap() {
                DataType::Number(n) => VisitorTypes::String(n.to_string()),
                DataType::String(s) => VisitorTypes::String(s.to_string()),
                DataType::Bool(_) => VisitorTypes::String("bool".to_string()),
                DataType::Nil => VisitorTypes::String("Nil".to_string()),
                DataType::Function(_) => VisitorTypes::String("Function".to_string()),
                DataType::Native(_) => VisitorTypes::String("Native".to_string()),
                DataType::Class(_) => VisitorTypes::String("Class".to_string()),
                DataType::Instance(_) => VisitorTypes::String("Instance".to_string()),
            }
        }
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> VisitorTypes {
        let expressions = vec![expr.left.as_ref(), expr.right.as_ref()];
        self.parenthesize(&expr.operator.lexeme, expressions)
    }

    fn visit_set_expr(&mut self, expr: &Set) -> VisitorTypes {
        let expressions = vec![expr.object.as_ref(), expr.value.as_ref()];
        self.parenthesize(&expr.name.lexeme, expressions)
    }

    fn visit_super_expr(&mut self, _expr: &Super) -> VisitorTypes {
        VisitorTypes::String("super".to_owned())
    }

    fn visit_this_expr(&mut self, _expr: &This) -> VisitorTypes {
        VisitorTypes::String("this".to_owned())
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> VisitorTypes {
        let expressions = vec![expr.right.as_ref()];
        self.parenthesize(&expr.operator.lexeme, expressions)
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> VisitorTypes {
        VisitorTypes::String(expr.name.lexeme.clone())
    }

    fn visit_assign_expr(&mut self, expr: &Assign) -> VisitorTypes {
        let expressions = vec![expr.value.as_ref()];
        self.parenthesize(&expr.name.lexeme, expressions)
    }
}
