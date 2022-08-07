use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::{
    ast::{
        Assign, Binary, Block, Call, Class, Expr, ExprVisitor, Expression, Function, Get, Grouping,
        If, Literal, Logical, Print, Return, Set, Stmt, StmtVisitor, Super, This, Unary, Var,
        Variable, VisitorTypes, While,
    },
    error,
    interpreter::Interpreter,
    token::Token,
};

pub struct Resolver<'a> {
    interpreter: &'a Interpreter,
    scopes: RefCell<Vec<RefCell<HashMap<String, bool>>>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: RefCell::new(Vec::new()),
        }
    }

    fn resolve(&mut self, statements: &Rc<Vec<Rc<dyn Stmt>>>) {
        for stmt in statements.iter() {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Rc<dyn Stmt>) {
        stmt.accept(self);
    }

    fn resolve_expr(&mut self, expr: &Rc<dyn Expr>) {
        expr.accept(self);
    }

    fn begin_scope(&mut self) {
        self.scopes.borrow_mut().push(RefCell::new(HashMap::new()));
    }

    fn end_scope(&mut self) {
        self.scopes.borrow_mut().pop();
    }

    fn declare(&mut self, name: Token) {
        if let Some(scope) = self.scopes.borrow().last() {
            scope.borrow_mut().insert(name.lexeme, false);
        }
    }

    fn define(&mut self, name: Token) {
        if let Some(scope) = self.scopes.borrow().last() {
            scope.borrow_mut().insert(name.lexeme, true);
        }
    }

    fn resolve_local(&mut self, expr: Rc<dyn Expr>, name: &Token) {
        for (scope, map) in self.scopes.borrow().iter().rev().enumerate() {
            if map.borrow().contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, scope);
                return;
            }
        }
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
        let token = expr.name.dup();
        if !self.scopes.borrow().is_empty()
            && self
                .scopes
                .borrow()
                .last()
                .unwrap()
                .borrow()
                .get(&token.dup().lexeme)
                == Some(&false)
        {
            error::resolve_error(&token, "Can't read local variable in its own initializer.");
        } else {
            let expr = expr;
            // TODO: Get it back to dyn somehow?
            self.resolve_local(Rc::clone(expr), &token);
        }
        VisitorTypes::Void(())
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> VisitorTypes {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
        VisitorTypes::Void(())
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
        self.declare(stmt.name.dup());
        if let Some(initializer) = &stmt.initializer {
            self.resolve_expr(initializer);
        }
        self.define(stmt.name.dup());
        VisitorTypes::Void(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> VisitorTypes {
        todo!()
    }
}
