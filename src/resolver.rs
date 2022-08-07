use crate::{
    ast::{
        Assign, Binary, Block, Call, Class, ExprVisitor, Expression, Function, Get, Grouping, If,
        Literal, Logical, Print, Return, Set, StmtVisitor, Super, This, Unary, Var, Variable,
        VisitorTypes, While,
    },
    interpreter::Interpreter,
};

pub struct Resolver<'a> {
    interpreter: &'a Interpreter,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a Interpreter) -> Resolver {
        Resolver { interpreter }
    }
}

impl<'a> ExprVisitor for Resolver<'a> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> VisitorTypes {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> VisitorTypes {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &Call) -> VisitorTypes {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &Get) -> VisitorTypes {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> VisitorTypes {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> VisitorTypes {
        todo!()
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
        todo!()
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> VisitorTypes {
        todo!()
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> VisitorTypes {
        todo!()
    }

    fn visit_class_stmt(&mut self, stmt: &Class) -> VisitorTypes {
        todo!()
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> VisitorTypes {
        todo!()
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> VisitorTypes {
        todo!()
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> VisitorTypes {
        todo!()
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> VisitorTypes {
        todo!()
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
