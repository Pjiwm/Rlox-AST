use crate::expr::*;

pub struct Interpreter {}

impl ExprVisitor for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> ReturnTypes {
        expr.value.accept(self)
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> ReturnTypes {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &Call) -> ReturnTypes {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &Get) -> ReturnTypes {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> ReturnTypes {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> ReturnTypes {
        todo!()
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> ReturnTypes {
        todo!()
    }

    fn visit_set_expr(&mut self, expr: &Set) -> ReturnTypes {
        todo!()
    }

    fn visit_super_expr(&mut self, expr: &Super) -> ReturnTypes {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &This) -> ReturnTypes {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> ReturnTypes {
        todo!()
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> ReturnTypes {
        todo!()
    }
}
