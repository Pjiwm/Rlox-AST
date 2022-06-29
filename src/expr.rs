use crate::token::{DataType, Token};

trait Expr {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R
    where
        Self: Sized;
}

trait Visitor<R> {
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

struct Assign {
    name: String,
    value: Box<dyn Expr>,
}
impl Assign {
    fn new(name: String, value: Box<dyn Expr>) -> Self {
        Self { name, value }
    }
}
impl Expr for Assign {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_assign_expr(self)
    }
}

struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}
impl Binary {
    fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
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

struct Call {
    callee: Box<dyn Expr>,
    paren: Token,
    arguments: Vec<Box<dyn Expr>>,
}
impl Call {
    fn new(callee: Box<dyn Expr>, paren: Token, arguments: Vec<Box<dyn Expr>>) -> Self {
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

struct Get {
    object: Box<dyn Expr>,
    name: Token,
}
impl Get {
    fn new(object: Box<dyn Expr>, name: Token) -> Self {
        Self { object, name }
    }
}
impl Expr for Get {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_get_expr(self)
    }
}

struct Grouping {
    expression: Box<dyn Expr>,
}
impl Grouping {
    fn new(expression: Box<dyn Expr>) -> Self {
        Self { expression }
    }
}
impl Expr for Grouping {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_grouping_expr(self)
    }
}

struct Literal {
    value: DataType,
}
impl Literal {
    fn new(value: DataType) -> Self {
        Self { value }
    }
}
impl Expr for Literal {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_literal_expr(self)
    }
}

struct Logical {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}
impl Logical {
    fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
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

struct Set {
    object: Box<dyn Expr>,
    name: Token,
    value: Box<dyn Expr>,
}
impl Set {
    fn new(object: Box<dyn Expr>, name: Token, value: Box<dyn Expr>) -> Self {
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

struct Super {
    keyword: Token,
    method: Token,
}
impl Super {
    fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }
}
impl Expr for Super {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_super_expr(self)
    }
}

struct This {
    keyword: Token,
}
impl This {
    fn new(keyword: Token) -> Self {
        Self { keyword }
    }
}
impl Expr for This {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_this_expr(self)
    }
}

struct Unary {
    operator: Token,
    right: Box<dyn Expr>,
}
impl Unary {
    fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Self { operator, right }
    }
}
impl Expr for Unary {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_unary_expr(self)
    }
}

struct Variable {
    name: Token,
}
impl Variable {
    fn new(name: Token) -> Self {
        Self { name }
    }
}
impl Expr for Variable {
    fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_variable_expr(self)
    }
}
