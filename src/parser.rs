use std::io::{self, Error, ErrorKind};

use crate::{
    error::parse_error,
    expr::{Binary, Expr, Grouping, Literal, Unary},
    token::{DataType, Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Box<dyn Expr>, Error> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Box<dyn Expr>, Error> {
        self.equality()
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
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, Error> {
        if self.check(token_type) {
            Ok(self.advance())
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
                // TODO self.advance might has to be used there followed by a return.
                _ => {}
            }
            self.advance();
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
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens.get(self.current -1).unwrap()
    }
}
