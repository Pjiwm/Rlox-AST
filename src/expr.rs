use crate::token::{DataType, Token};

pub trait Expr {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R
    where
        Self: Sized;
}

pub trait Visitor<R> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> R;
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn visit_call_expr(&mut self, expr: &Call) -> R;
    fn visit_get_expr(&mut self, expr: &Get) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literal_expr(&mut self, expr: &Literal) -> R;
    fn visit_logical_expr(&mut self, expr: &Logical) -> R;
    fn visit_set_expr(&mut self, expr: &Set) -> R;
    fn visit_super_expr(&mut self, expr: &Super) -> R;
    fn visit_this_expr(&mut self, expr: &This) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
    fn visit_variable_expr(&mut self, expr: &Variable) -> R;
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
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_assign_expr(self)
    }
}

pub struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
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
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
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
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
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
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_get_expr(self)
    }
}

pub struct Grouping {
    expression: Box<dyn Expr>,
}
impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self { expression }
    }
}
impl Expr for Grouping {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_grouping_expr(self)
    }
}

pub struct Literal {
    value: DataType,
}
impl Literal {
    pub fn new(value: DataType) -> Self {
        Self { value }
    }
}
impl Expr for Literal {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
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
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
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
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
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
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
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
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_this_expr(self)
    }
}

pub struct Unary {
    operator: Token,
    right: Box<dyn Expr>,
}
impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Self { operator, right }
    }
}
impl Expr for Unary {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
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
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_variable_expr(self)
    }
}
