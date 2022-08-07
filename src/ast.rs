use std::{any::Any, rc::Rc};

use crate::token::{DataType, Token};

pub trait Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes;
    fn as_any(&self) -> &dyn Any;
}
#[derive(Debug)]
pub enum VisitorTypes {
    String(String),
    DataType(Option<DataType>),
    RunTimeError { token: Option<Token>, msg: String },
    Return(Option<DataType>),
    Void(()),
}

pub trait ExprVisitor {
    fn visit_assign_expr(&mut self, expr: &Assign) -> VisitorTypes;
    fn visit_binary_expr(&mut self, expr: &Binary) -> VisitorTypes;
    fn visit_call_expr(&mut self, expr: &Call) -> VisitorTypes;
    fn visit_get_expr(&mut self, expr: &Get) -> VisitorTypes;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> VisitorTypes;
    fn visit_literal_expr(&mut self, expr: &Literal) -> VisitorTypes;
    fn visit_logical_expr(&mut self, expr: &Logical) -> VisitorTypes;
    fn visit_set_expr(&mut self, expr: &Set) -> VisitorTypes;
    fn visit_super_expr(&mut self, expr: &Super) -> VisitorTypes;
    fn visit_this_expr(&mut self, expr: &This) -> VisitorTypes;
    fn visit_unary_expr(&mut self, expr: &Unary) -> VisitorTypes;
    fn visit_variable_expr(&mut self, expr: &Variable) -> VisitorTypes;
}

pub struct Assign {
    pub name: Token,
    pub value: Rc<dyn Expr>,
}
impl Assign {
    pub fn new(name: Token, value: Rc<dyn Expr>) -> Self {
        Self { name, value }
    }
}
impl Expr for Assign {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_assign_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Binary {
    pub left: Rc<dyn Expr>,
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}
impl Binary {
    pub fn new(left: Rc<dyn Expr>, operator: Token, right: Rc<dyn Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
impl Expr for Binary {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_binary_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Call {
    pub callee: Rc<dyn Expr>,
    pub paren: Token,
    pub arguments: Vec<Rc<dyn Expr>>,
}
impl Call {
    pub fn new(callee: Rc<dyn Expr>, paren: Token, arguments: Vec<Rc<dyn Expr>>) -> Self {
        Self {
            callee,
            paren,
            arguments,
        }
    }
}
impl Expr for Call {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_call_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Get {
    pub object: Rc<dyn Expr>,
    pub name: Token,
}
impl Get {
    pub fn new(object: Rc<dyn Expr>, name: Token) -> Self {
        Self { object, name }
    }
}
impl Expr for Get {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_get_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Grouping {
    pub expression: Rc<dyn Expr>,
}
impl Grouping {
    pub fn new(expression: Rc<dyn Expr>) -> Self {
        Self { expression }
    }
}
impl Expr for Grouping {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_grouping_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Literal {
    pub value: Option<DataType>,
}
impl Literal {
    pub fn new(value: Option<DataType>) -> Self {
        Self { value: value }
    }
}
impl Expr for Literal {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_literal_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Logical {
    pub left: Rc<dyn Expr>,
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}
impl Logical {
    pub fn new(left: Rc<dyn Expr>, operator: Token, right: Rc<dyn Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
impl Expr for Logical {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_logical_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Set {
    pub object: Rc<dyn Expr>,
    pub name: Token,
    pub value: Rc<dyn Expr>,
}
impl Set {
    pub fn new(object: Rc<dyn Expr>, name: Token, value: Rc<dyn Expr>) -> Self {
        Self {
            object,
            name,
            value,
        }
    }
}
impl Expr for Set {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_set_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Super {
    pub keyword: Token,
    pub method: Token,
}
impl Super {
    pub fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }
}
impl Expr for Super {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_super_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct This {
    pub keyword: Token,
}
impl This {
    pub fn new(keyword: Token) -> Self {
        Self { keyword }
    }
}
impl Expr for This {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_this_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}
impl Unary {
    pub fn new(operator: Token, right: Rc<dyn Expr>) -> Self {
        Self { operator, right }
    }
}
impl Expr for Unary {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_unary_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Variable {
    pub name: Token,
}
impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}
impl Expr for Variable {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> VisitorTypes {
        visitor.visit_variable_expr(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait Stmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> VisitorTypes;
}

pub trait StmtVisitor {
    fn visit_block_stmt(&mut self, stmt: &Block) -> VisitorTypes;
    fn visit_class_stmt(&mut self, stmt: &Class) -> VisitorTypes;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> VisitorTypes;
    fn visit_function_stmt(&mut self, stmt: &Function) -> VisitorTypes;
    fn visit_if_stmt(&mut self, stmt: &If) -> VisitorTypes;
    fn visit_print_stmt(&mut self, stmt: &Print) -> VisitorTypes;
    fn visit_return_stmt(&mut self, stmt: &Return) -> VisitorTypes;
    fn visit_var_stmt(&mut self, stmt: &Var) -> VisitorTypes;
    fn visit_while_stmt(&mut self, stmt: &While) -> VisitorTypes;
}
pub struct Block {
    pub statements: Rc<Vec<Rc<dyn Stmt>>>,
}
impl Block {
    pub fn new(statements: Rc<Vec<Rc<dyn Stmt>>>) -> Self {
        Self { statements }
    }
}
impl Stmt for Block {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> VisitorTypes {
        visitor.visit_block_stmt(self)
    }
}

pub struct Class {
    pub name: Token,
    // Check if these works, cause they might not...
    pub methods: Vec<Rc<Function>>,
    pub super_class: Option<Rc<Variable>>,
}
impl Class {
    pub fn new(
        name: Token,
        methods: Vec<Rc<Function>>,
        super_class: Option<Rc<Variable>>,
    ) -> Self {
        Self {
            name,
            methods,
            super_class,
        }
    }
}
impl Stmt for Class {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> VisitorTypes {
        visitor.visit_class_stmt(self)
    }
}

pub struct Expression {
    pub expression: Rc<dyn Expr>,
}
impl Expression {
    pub fn new(expression: Rc<dyn Expr>) -> Self {
        Self { expression }
    }
}
impl Stmt for Expression {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> VisitorTypes {
        visitor.visit_expression_stmt(self)
    }
}

pub struct Function {
    pub name: Token,
    pub params: Rc<Vec<Token>>,
    pub body: Rc<Vec<Rc<dyn Stmt>>>,
}
impl Function {
    pub fn new(name: Token, param: Rc<Vec<Token>>, body: Rc<Vec<Rc<dyn Stmt>>>) -> Self {
        Self { name, params: param, body }
    }
}
impl Stmt for Function {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> VisitorTypes {
        visitor.visit_function_stmt(self)
    }
}

pub struct If {
    pub condition: Rc<dyn Expr>,
    pub then_branch: Rc<dyn Stmt>,
    pub else_branch: Option<Rc<dyn Stmt>>,
}
impl If {
    pub fn new(
        condition: Rc<dyn Expr>,
        then_branch: Rc<dyn Stmt>,
        else_branch: Option<Rc<dyn Stmt>>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}
impl Stmt for If {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> VisitorTypes {
        visitor.visit_if_stmt(self)
    }
}

pub struct Print {
    pub expression: Rc<dyn Expr>,
}
impl Print {
    pub fn new(expression: Rc<dyn Expr>) -> Self {
        Self { expression }
    }
}
impl Stmt for Print {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> VisitorTypes {
        visitor.visit_print_stmt(self)
    }
}

pub struct Return {
    pub keyword: Token,
    pub value: Option<Rc<dyn Expr>>,
}

impl Return {
    pub fn new(keyword: Token, value: Option<Rc<dyn Expr>>) -> Self {
        Self { keyword, value }
    }
}
impl Stmt for Return {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> VisitorTypes {
        visitor.visit_return_stmt(self)
    }
}

pub struct Var {
    pub name: Token,
    pub initializer: Option<Rc<dyn Expr>>,
}
impl Var {
    pub fn new(name: Token, initializer: Option<Rc<dyn Expr>>) -> Self {
        Self { name, initializer }
    }
}
impl Stmt for Var {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> VisitorTypes {
        visitor.visit_var_stmt(self)
    }
}

pub struct While {
    pub condition: Rc<dyn Expr>,
    pub body: Rc<dyn Stmt>,
}
impl While {
    pub fn new(condition: Rc<dyn Expr>, body: Rc<dyn Stmt>) -> Self {
        Self { condition, body }
    }
}
impl Stmt for While {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> VisitorTypes {
        visitor.visit_while_stmt(self)
    }
}
