use crate::{expr::{Binary, Expr, ExprVisitor, ReturnTypes}, token::DataType};

pub struct AstPrinter;
impl AstPrinter {
    pub fn new() -> Self {
        Self
    }

    pub fn print<T: Expr>(&mut self, expr: &mut T) -> String {
        let return_string = match expr.accept(self) {
            ReturnTypes::String(s) => s,
        };
        return_string 
    }

    fn paranthesize(&mut self, name: &String, exprs: Vec<&dyn Expr>) -> String {
        let mut s = String::new();
        s.push_str("(");
        s.push_str(name);
        for expr in exprs {
            s.push_str(" ");
            let expr_str = match  expr.accept(self) {
                ReturnTypes::String(s) => s,
                _ => "Invalid Expression".to_string(),
            };
            s.push_str(expr_str.as_str());
        }
        s.push_str(")");
        s
    }
}

impl ExprVisitor for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> ReturnTypes {
        let expressions = vec![expr.left.as_ref(), expr.right.as_ref()];
        ReturnTypes::String(self.paranthesize(&expr.operator.lexeme, expressions))
    }

    fn visit_call_expr(&mut self, expr: &crate::expr::Call) -> ReturnTypes {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &crate::expr::Get) -> ReturnTypes {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &crate::expr::Grouping) -> ReturnTypes {
        let expressions = vec![expr.expression.as_ref()];
        ReturnTypes::String(self.paranthesize(&"group".to_owned(), expressions))
    }

    fn visit_literal_expr(&mut self, expr: &crate::expr::Literal) -> ReturnTypes {
        if expr.value.is_none() {
            ReturnTypes::String("nil".to_owned())
        } else {
            match expr.value.as_ref().unwrap() {
                DataType::Number(n) => ReturnTypes::String(n.to_string()),
                DataType::String(s) => ReturnTypes::String(s.to_string()),
            }
        }
    }

    fn visit_logical_expr(&mut self, expr: &crate::expr::Logical) -> ReturnTypes {
        todo!()
    }

    fn visit_set_expr(&mut self, expr: &crate::expr::Set) -> ReturnTypes {
        todo!()
    }

    fn visit_super_expr(&mut self, expr: &crate::expr::Super) -> ReturnTypes {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &crate::expr::This) -> ReturnTypes {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &crate::expr::Unary) -> ReturnTypes {
        let expresssions = vec![expr.right.as_ref()];
        ReturnTypes::String(self.paranthesize(&expr.operator.lexeme, expresssions))
    }

    fn visit_variable_expr(&mut self, expr: &crate::expr::Variable) -> ReturnTypes {
        todo!()
    }

    fn visit_assign_expr(&mut self, expr: &crate::expr::Assign) -> ReturnTypes {
        todo!()
    }
}
