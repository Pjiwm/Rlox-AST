use crate::expr::{Expr, ExprVisitor, Binary};

pub struct AstPrinter;
impl AstPrinter {
    // pub fn print(&mut self, expr: &dyn Expr) {
    //     expr.accept(&self);
    // }
    pub fn new() -> Self {
        Self
    }

    pub fn print<T: Expr>(&mut self, expr: &T) {
        expr.accept(self);
    }

    fn paranthesize<T: Expr>(&mut self, name: String, exprs: &[T]) -> String {
        let mut s = String::new();
        s.push_str("(");
        for expr in exprs {
            s.push_str(" ");
            s.push_str(expr.accept::<String>(self).as_str());
        }
        s.push_str(")");
        s
    }
}

impl ExprVisitor<String> for AstPrinter {
    // TODO fix this...
    fn visit_binary_expr(&mut self, expr: &Binary) -> String {
        self.paranthesize::<dyn Expr>(expr.operator.lexeme, [expr.left.as_mut(), expr.right.as_mut()])
    }

    fn visit_call_expr(&mut self, expr: &crate::expr::Call) -> String {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &crate::expr::Get) -> String {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &crate::expr::Grouping) -> String {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &crate::expr::Literal) -> String {
        todo!()
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
        todo!()
    }

    fn visit_variable_expr(&mut self, expr: &crate::expr::Variable) -> String {
        todo!()
    }

    fn visit_assign_expr(&mut self, expr: &crate::expr::Assign) -> String {
        todo!()
    }
}
