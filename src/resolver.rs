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

    pub fn resolve(&mut self, statements: &Rc<Vec<Rc<dyn Stmt>>>) {
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

    fn resolve_local(&mut self, expr: Rc<dyn Expr>, name: &Token) {
        for (scope, map) in self.scopes.borrow().iter().rev().enumerate() {
            if map.borrow().contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, scope);
                return;
            }
        }
    }

    fn resolve_function(&mut self, func: &Function) {
        self.begin_scope();
        for param in func.params.iter() {
            self.declare(param.dup());
            self.define(param.dup());
        }
        self.resolve(&func.body);
        self.end_scope();
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
}

impl<'a> ExprVisitor for Resolver<'a> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> VisitorTypes {
        let name = expr.name.dup();
        let value = expr.value.clone();
        let expr: Rc<dyn Expr> = Rc::new(Assign::new(expr.name.dup(), expr.value.clone()));
        self.resolve_expr(&value);
        self.resolve_local(expr, &name);
        VisitorTypes::Void(())
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> VisitorTypes {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
        VisitorTypes::Void(())
    }

    fn visit_call_expr(&mut self, expr: &Call) -> VisitorTypes {
        self.resolve_expr(&expr.callee);
        for arg in expr.arguments.iter() {
            self.resolve_expr(arg);
        }
        VisitorTypes::Void(())
    }

    fn visit_get_expr(&mut self, expr: &Get) -> VisitorTypes {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> VisitorTypes {
        self.resolve_expr(&expr.expression);
        VisitorTypes::Void(())
    }

    fn visit_literal_expr(&mut self, _: &Literal) -> VisitorTypes {
        VisitorTypes::Void(())
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> VisitorTypes {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
        VisitorTypes::Void(())
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
        self.resolve_expr(&expr.right);
        VisitorTypes::Void(())
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
            // TODO: Check if this causes bugs or not.
            let expr: Rc<dyn Expr> = Rc::new(Variable::new(expr.name.dup()));
            self.resolve_local(Rc::clone(&expr), &token);
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
        self.resolve_expr(&stmt.expression);
        VisitorTypes::Void(())
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> VisitorTypes {
        let name = stmt.name.dup();
        self.declare(name.dup());
        self.define(name);
        self.resolve_function(stmt);
        VisitorTypes::Void(())
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> VisitorTypes {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.then_branch);
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_stmt(&else_branch);
        }
        VisitorTypes::Void(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> VisitorTypes {
        self.resolve_expr(&stmt.expression);
        VisitorTypes::Void(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> VisitorTypes {
        if let Some(value) = &stmt.value {
            self.resolve_expr(value);
        }
        VisitorTypes::Void(())
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
        self.resolve_expr(&stmt.condition.clone());
        self.resolve_stmt(&stmt.body.clone());
        VisitorTypes::Void(())
    }
}
