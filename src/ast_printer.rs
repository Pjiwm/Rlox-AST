use crate::{expr::{Binary, Expr, ExprVisitor}, token::DataType};

pub struct AstPrinter;
impl AstPrinter {
    pub fn new() -> Self {
        Self
    }

    pub fn print<T: Expr>(&mut self, expr: &mut T) -> String {
        expr.accept(self)
    }

    fn paranthesize(&mut self, name: &String, exprs: Vec<&dyn Expr>) -> String {
        let mut s = String::new();
        s.push_str("(");
        s.push_str(name);
        for mut expr in exprs {
            s.push_str(" ");
            s.push_str(expr.accept::<String>(self).as_str());
        }
        s.push_str(")");
        s
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &mut Binary) -> String {
        let expressions = vec![expr.left.as_ref(), expr.right.as_ref()];
        self.paranthesize(&expr.operator.lexeme, expressions)
    }

    fn visit_call_expr(&mut self, expr: &crate::expr::Call) -> String {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &crate::expr::Get) -> String {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &crate::expr::Grouping) -> String {
        let expressions = vec![expr.expression.as_ref()];
        self.paranthesize(&"group".to_owned(), expressions)
    }

    fn visit_literal_expr(&mut self, expr: &crate::expr::Literal) -> String {
        if expr.value.is_some() {
            "nill".to_owned()
        } else {
            match expr.value.as_ref().unwrap() {
                DataType::Number(n) => n.to_string(),
                DataType::String(s) => s.to_string(),
            }
        }
    }

    fn visit_logical_expr(&mut self, expr: &crate::expr::Logical) -> String {
        todo!()
    }

    fn visit_set_expr(&mut self, expr: &crate::expr::Set) -> String {
        todo!()
    }

    fn visit_super_expr(&mut self, expr: &crate::expr::Super) -> String {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &crate::expr::This) -> String {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &crate::expr::Unary) -> String {
        let expresssions = vec![expr.right.as_ref()];
        self.paranthesize(&expr.operator.lexeme, expresssions)
    }

    fn visit_variable_expr(&mut self, expr: &crate::expr::Variable) -> String {
        todo!()
    }

    fn visit_assign_expr(&mut self, expr: &crate::expr::Assign) -> String {
        todo!()
    }
}
