use crate::token::{DataType, Token};

pub trait Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> ReturnTypes;
}
pub enum ReturnTypes {
    String(String),
}

pub trait ExprVisitor {
    fn visit_assign_expr(&mut self, expr: &Assign) -> ReturnTypes;
    fn visit_binary_expr(&mut self, expr: &Binary) -> ReturnTypes;
    fn visit_call_expr(&mut self, expr: &Call) -> ReturnTypes;
    fn visit_get_expr(&mut self, expr: &Get) -> ReturnTypes;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> ReturnTypes;
    fn visit_literal_expr(&mut self, expr: &Literal) -> ReturnTypes;
    fn visit_logical_expr(&mut self, expr: &Logical) -> ReturnTypes;
    fn visit_set_expr(&mut self, expr: &Set) -> ReturnTypes;
    fn visit_super_expr(&mut self, expr: &Super) -> ReturnTypes;
    fn visit_this_expr(&mut self, expr: &This) -> ReturnTypes;
    fn visit_unary_expr(&mut self, expr: &Unary) -> ReturnTypes;
    fn visit_variable_expr(&mut self, expr: &Variable) -> ReturnTypes;
}

pub struct Assign {
    name: String,
    value: Box<dyn Expr>,
}
impl Assign {
    pub fn new(name: String, value: Box<dyn Expr>) -> Self {
        Self { name, value }
    }
}
impl Expr for Assign {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_assign_expr(self)
    }
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}
impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
impl Expr for Binary {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_binary_expr(self)
    }
}

pub struct Call {
    callee: Box<dyn Expr>,
    paren: Token,
    arguments: Vec<Box<dyn Expr>>,
}
impl Call {
    pub fn new(callee: Box<dyn Expr>, paren: Token, arguments: Vec<Box<dyn Expr>>) -> Self {
        Self {
            callee,
            paren,
            arguments,
        }
    }
}
impl Expr for Call {
    fn accept(& self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_call_expr(self)
    }
}

pub struct Get {
    object: Box<dyn Expr>,
    name: Token,
}
impl Get {
    pub fn new(object: Box<dyn Expr>, name: Token) -> Self {
        Self { object, name }
    }
}
impl Expr for Get {
    fn accept(& self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_get_expr(self)
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expr>,
}
impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self { expression }
    }
}
impl Expr for Grouping {
    fn accept(& self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_grouping_expr(self)
    }
}

pub struct Literal {
    pub value: Option<DataType>,
}
impl Literal {
    pub fn new(value: DataType) -> Self {
        Self { value: Some(value) }
    }
}
impl Expr for Literal {
    fn accept(& self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_literal_expr(self)
    }
}

pub struct Logical {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}
impl Logical {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
impl Expr for Logical {
    fn accept(& self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_logical_expr(self)
    }
}

pub struct Set {
    object: Box<dyn Expr>,
    name: Token,
    value: Box<dyn Expr>,
}
impl Set {
    pub fn new(object: Box<dyn Expr>, name: Token, value: Box<dyn Expr>) -> Self {
        Self {
            object,
            name,
            value,
        }
    }
}
impl Expr for Set {
    fn accept(& self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_set_expr(self)
    }
}

pub struct Super {
    keyword: Token,
    method: Token,
}
impl Super {
    pub fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }
}
impl Expr for Super {
    fn accept(& self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_super_expr(self)
    }
}

pub struct This {
    keyword: Token,
}
impl This {
    pub fn new(keyword: Token) -> Self {
        Self { keyword }
    }
}
impl Expr for This {
    fn accept(& self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_this_expr(self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}
impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Self { operator, right }
    }
}
impl Expr for Unary {
    fn accept(& self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_unary_expr(self)
    }
}

pub struct Variable {
    name: Token,
}
impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}
impl Expr for Variable {
    fn accept(& self, visitor: &mut dyn ExprVisitor) -> ReturnTypes {
        visitor.visit_variable_expr(self)
    }
}

pub trait Stmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ReturnTypes
    where
        Self: Sized;
}

pub trait StmtVisitor {
    fn visit_block_stmt(&mut self, stmt: &Block) -> ReturnTypes;
    fn visit_class_stmt(&mut self, stmt: &Class) -> ReturnTypes;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> ReturnTypes;
    fn visit_function_stmt(&mut self, stmt: &Function) -> ReturnTypes;
    fn visit_if_stmt(&mut self, stmt: &If) -> ReturnTypes;
    fn visit_print_stmt(&mut self, stmt: &Print) -> ReturnTypes;
    fn visit_return_stmt(&mut self, stmt: &Return) -> ReturnTypes;
    fn visit_var_stmt(&mut self, stmt: &Var) -> ReturnTypes;
    fn visit_while_stmt(&mut self, stmt: &While) -> ReturnTypes;
}

pub struct Block {
    statements: Vec<Box<dyn Stmt>>,
}
impl Block {
    pub fn new(statements: Vec<Box<dyn Stmt>>) -> Self {
        Self { statements }
    }
}
impl Stmt for Block {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ReturnTypes {
        visitor.visit_block_stmt(self)
    }
}

pub struct Class {
    name: Token,
    // Check if these works, cause they might not...
    methods: Vec<Box<Function>>,
    super_class: Option<Box<Variable>>,
}
impl Class {
    pub fn new(
        name: Token,
        methods: Vec<Box<Function>>,
        super_class: Option<Box<Variable>>,
    ) -> Self {
        Self {
            name,
            methods,
            super_class,
        }
    }
}
impl Stmt for Class {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ReturnTypes {
        visitor.visit_class_stmt(self)
    }
}

pub struct Expression {
    expression: Box<dyn Expr>,
}
impl Expression {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self { expression }
    }
}
impl Stmt for Expression {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ReturnTypes {
        visitor.visit_expression_stmt(self)
    }
}

pub struct Function {
    name: Token,
    param: Vec<Token>,
    body: Box<dyn Stmt>,
}
impl Function {
    pub fn new(name: Token, param: Vec<Token>, body: Box<dyn Stmt>) -> Self {
        Self { name, param, body }
    }
}
impl Stmt for Function {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ReturnTypes {
        visitor.visit_function_stmt(self)
    }
}

pub struct If {
    condition: Box<dyn Expr>,
    then_branch: Box<dyn Stmt>,
    else_branch: Option<Box<dyn Stmt>>,
}
impl If {
    pub fn new(
        condition: Box<dyn Expr>,
        then_branch: Box<dyn Stmt>,
        else_branch: Option<Box<dyn Stmt>>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}
impl Stmt for If {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ReturnTypes {
        visitor.visit_if_stmt(self)
    }
}

pub struct Print {
    expression: Box<dyn Expr>,
}
impl Print {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self { expression }
    }
}
impl Stmt for Print {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ReturnTypes {
        visitor.visit_print_stmt(self)
    }
}

pub struct Return {
    keyword: Token,
    // TODO For later: Find out if option is really needed?
    value: Option<Box<dyn Expr>>,
}

impl Return {
    pub fn new(keyword: Token, value: Option<Box<dyn Expr>>) -> Self {
        Self { keyword, value }
    }
}
impl Stmt for Return {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ReturnTypes {
        visitor.visit_return_stmt(self)
    }
}

pub struct Var {
    name: Token,
    initializer: Option<Box<dyn Expr>>,
}
impl Var {
    pub fn new(name: Token, initializer: Option<Box<dyn Expr>>) -> Self {
        Self { name, initializer }
    }
}
impl Stmt for Var {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ReturnTypes {
        visitor.visit_var_stmt(self)
    }
}

pub struct While {
    condition: Box<dyn Expr>,
    body: Box<dyn Stmt>,
}
impl While {
    pub fn new(condition: Box<dyn Expr>, body: Box<dyn Stmt>) -> Self {
        Self { condition, body }
    }
}
impl Stmt for While {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> ReturnTypes {
        visitor.visit_while_stmt(self)
    }
}