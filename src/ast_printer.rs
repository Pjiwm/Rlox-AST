use crate::{expr::*, token::DataType};

pub struct AstPrinter;
impl AstPrinter {
    pub fn new() -> Self {
        Self
    }

    pub fn print(&mut self, expr: Box<dyn Expr>) -> String {
        let return_string = match expr.accept(self) {
            ReturnTypes::String(s) => s,
            ReturnTypes::DataType(_) => "Incorrect expression".to_string(),
        };
        return_string
    }

    fn paranthesize(&mut self, name: &String, exprs: Vec<&dyn Expr>) -> ReturnTypes {
        let mut s = String::new();
        s.push_str("(");
        s.push_str(name);
        for expr in exprs {
            s.push_str(" ");
            let expr_str = match expr.accept(self) {
                ReturnTypes::String(s) => s,
                ReturnTypes::DataType(_) => "Incorrect expression".to_string(),
            };
            s.push_str(expr_str.as_str());
        }
        s.push_str(")");
        ReturnTypes::String(s)
    }
}

impl ExprVisitor for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> ReturnTypes {
        let expressions = vec![expr.left.as_ref(), expr.right.as_ref()];
        self.paranthesize(&expr.operator.lexeme, expressions)
    }

    fn visit_call_expr(&mut self, expr: &Call) -> ReturnTypes {
        let mut expressions = vec![expr.callee.as_ref()];
        for arg in &expr.arguments {
            expressions.push(arg.as_ref());
        }
        self.paranthesize(&expr.paren.lexeme, expressions)
    }

    fn visit_get_expr(&mut self, expr: &Get) -> ReturnTypes {
        let expressions = vec![expr.object.as_ref()];
        self.paranthesize(&expr.name.lexeme, expressions)
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> ReturnTypes {
        let expressions = vec![expr.expression.as_ref()];
        self.paranthesize(&"group".to_owned(), expressions)
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> ReturnTypes {
        if expr.value.is_none() {
            ReturnTypes::String("nil".to_owned())
        } else {
            match expr.value.as_ref().unwrap() {
                DataType::Number(n) => ReturnTypes::String(n.to_string()),
                DataType::String(s) => ReturnTypes::String(s.to_string()),
                DataType::Bool(_) => ReturnTypes::String("bool".to_string()),
                DataType::Nil => ReturnTypes::String("Nil".to_string()),
            }
        }
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> ReturnTypes {
        let expressions = vec![expr.left.as_ref(), expr.right.as_ref()];
        self.paranthesize(&expr.operator.lexeme, expressions)
    }

    fn visit_set_expr(&mut self, expr: &Set) -> ReturnTypes {
        let expressions = vec![expr.object.as_ref(), expr.value.as_ref()];
        self.paranthesize(&expr.name.lexeme, expressions)
    }

    fn visit_super_expr(&mut self, _expr: &Super) -> ReturnTypes {
        ReturnTypes::String("super".to_owned())
    }

    fn visit_this_expr(&mut self, _expr: &This) -> ReturnTypes {
        ReturnTypes::String("this".to_owned())
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> ReturnTypes {
        let expresssions = vec![expr.right.as_ref()];
        self.paranthesize(&expr.operator.lexeme, expresssions)
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> ReturnTypes {
        ReturnTypes::String(expr.name.lexeme.clone())
    }

    fn visit_assign_expr(&mut self, expr: &Assign) -> ReturnTypes {
        let expressions = vec![expr.value.as_ref()];
        self.paranthesize(&expr.name, expressions)
    }
}
