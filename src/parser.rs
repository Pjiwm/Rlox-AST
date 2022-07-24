use std::io::{self, Error, ErrorKind};

use crate::{
    ast::{
        Assign, Binary, Block, Expr, Expression, Grouping, If, Literal, Logical, Print, Stmt,
        Unary, Var, Variable,
    },
    error::parse_error,
    token::{DataType, Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &[Token]) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Stmt>>, Error> {
        let mut statements = Vec::<Box<dyn Stmt>>::new();
        while !self.is_at_end() {
            statements.push(self.decleration()?);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Box<dyn Expr>, Error> {
        self.assignment()
    }

    fn decleration(&mut self) -> Result<Box<dyn Stmt>, Error> {
        let var_vec = vec![TokenType::Var];
        if self.matches(&var_vec) {
            return self.var_decleration();
        }
        match self.statement() {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.synchronize();
                Err(err)
            }
        }
    }

    fn statement(&mut self) -> Result<Box<dyn Stmt>, Error> {
        let print_vec = vec![TokenType::Print];
        let block_vec = vec![TokenType::LeftBrace];
        let if_vec = vec![TokenType::If];
        if self.matches(&if_vec) {
            self.if_statement()
        } else if self.matches(&print_vec) {
            self.print_statement()
        } else if self.matches(&block_vec) {
            Ok(Box::new(Block::new(self.block()?)))
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Box<dyn Stmt>, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch: Option<Box<dyn Stmt>> = None;
        let else_vec = vec![TokenType::Else];
        if self.matches(&else_vec) {
            // TODO this might become a bug.
            else_branch = Some(match self.statement() {
                Ok(e) => e,
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Expect statement after 'else'.",
                    ));
                }
            });
        }
        Ok(Box::new(If::new(expr, then_branch, else_branch)))
    }

    fn print_statement(&mut self) -> Result<Box<dyn Stmt>, Error> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Box::new(Print::new(value)))
    }

    fn var_decleration(&mut self) -> Result<Box<dyn Stmt>, Error> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let equal_vec = vec![TokenType::Equal];
        let initializer = if self.matches(&equal_vec) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Box::new(Var::new(name.dup(), initializer)))
    }

    fn expression_statement(&mut self) -> Result<Box<dyn Stmt>, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Box::new(Expression::new(expr)))
    }

    fn block(&mut self) -> Result<Box<Vec<Box<dyn Stmt>>>, Error> {
        let mut statements = Vec::<Box<dyn Stmt>>::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.decleration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(Box::new(statements))
    }

    fn assignment(&mut self) -> Result<Box<dyn Expr>, Error> {
        // let expr = self.equality()?;
        let expr = self.or()?;
        let equal_vec = vec![TokenType::Equal];
        if self.matches(&equal_vec) {
            let equals = self.previous().dup();
            let value = self.assignment()?;
            match expr.as_any().downcast_ref::<Variable>() {
                Some(v) => return Ok(Box::new(Assign::new(v.name.dup(), value))),
                None => return Err(self.parse_error(&equals, "Invalid assignment target.")),
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Box<dyn Expr>, Error> {
        let mut expr = self.and()?;

        let or_vec = vec![TokenType::Or];
        while self.matches(&or_vec) {
            let operator = self.previous().dup();
            let right = self.and()?;
            expr = Box::new(Logical::new(expr, operator, right))
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Box<dyn Expr>, Error> {
        let mut expr = self.equality()?;

        let and_vec = vec![TokenType::And];
        while self.matches(&and_vec) {
            let operator = self.previous().dup();
            let right = self.equality()?;
            expr = Box::new(Logical::new(expr, operator, right))
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<dyn Expr>, Error> {
        let mut expr = self.comparison();
        let equal_vec = vec![TokenType::Equalequal, TokenType::Bangequal];
        while self.matches(&equal_vec) {
            let operator = self.previous().dup();
            let right = self.comparison();
            expr = Ok(Box::new(Binary::new(expr?, operator.clone(), right?)));
        }
        expr
    }

    fn comparison(&mut self) -> Result<Box<dyn Expr>, Error> {
        let mut expr = self.term();
        let comparison_vec = vec![
            TokenType::Greater,
            TokenType::Greaterequal,
            TokenType::Less,
            TokenType::Lessequal,
        ];
        while self.matches(&comparison_vec) {
            let operator = self.previous().dup();
            let right = self.term();
            expr = Ok(Box::new(Binary::new(expr?, operator.clone(), right?)));
        }
        expr
    }

    fn term(&mut self) -> Result<Box<dyn Expr>, Error> {
        let mut expr = self.factor();
        let term_vec = vec![TokenType::Minus, TokenType::Plus];
        while self.matches(&term_vec) {
            let operator = self.previous().dup();
            let right = self.factor();
            expr = Ok(Box::new(Binary::new(expr?, operator.clone(), right?)));
        }
        expr
    }

    fn factor(&mut self) -> Result<Box<dyn Expr>, Error> {
        let mut expr = self.unary();
        let factor_vec = vec![TokenType::Star, TokenType::Slash];
        while self.matches(&factor_vec) {
            let operator = self.previous().dup();
            let right = self.unary();
            expr = Ok(Box::new(Binary::new(expr?, operator.clone(), right?)));
        }
        expr
    }

    fn unary(&mut self) -> Result<Box<dyn Expr>, Error> {
        let unary_vec = vec![TokenType::Bang, TokenType::Minus];
        if self.matches(&unary_vec) {
            let operator = self.previous().dup();
            let right = self.unary();
            return Ok(Box::new(Unary::new(operator.clone(), right?)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Box<dyn Expr>, Error> {
        let false_vec = vec![TokenType::False];
        if self.matches(&false_vec) {
            return Ok(Box::new(Literal::new(Some(DataType::Bool(false)))));
        }
        let true_vec = vec![TokenType::True];
        if self.matches(&true_vec) {
            return Ok(Box::new(Literal::new(Some(DataType::Bool(true)))));
        }
        let nil_vec = vec![TokenType::Nil];
        if self.matches(&nil_vec) {
            return Ok(Box::new(Literal::new(None)));
        }

        let data_type_vec = vec![TokenType::Number, TokenType::String];
        if self.matches(&data_type_vec) {
            let data_type = self.previous();
            return Ok(Box::new(Literal::new(Some(
                data_type.clone().literal.unwrap(),
            ))));
        }
        let identifier_vec = vec![TokenType::Identifier];
        if self.matches(&identifier_vec) {
            return Ok(Box::new(Variable::new(self.previous().dup())));
        }

        let left_paren_vec = vec![TokenType::LeftParen];
        if self.matches(&left_paren_vec) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Box::new(Grouping::new(expr?)));
        }

        Err(self.parse_error(self.peek(), "Expect expression."))
    }

    fn matches(&mut self, types: &Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            Ok(self.advance().dup())
        } else {
            Err(self.parse_error(self.peek(), message))
        }
    }

    fn parse_error(&self, token: &Token, message: &str) -> Error {
        parse_error(token, message);
        io::Error::new(ErrorKind::Other, message)
    }

    fn synchronize(&mut self) {
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {
                    self.advance();
                    ()
                }
            }
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
