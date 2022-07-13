use std::io::{self, Error, ErrorKind};

use crate::{
    error::token_error,
    expr::{Binary, Expr, Grouping, Literal, Unary},
    token::{DataType, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    // TODO for later:
    // Every function in here might have to return a Result type and we handle that within this parse function.
    pub fn parse(&mut self) -> Box<dyn Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Box<dyn Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<dyn Expr> {
        let mut expr: Box<dyn Expr> = self.comparison();
        let equal_vec = vec![TokenType::Equalequal, TokenType::Bangequal];
        while self.matches(&equal_vec) {
            let right = self.comparison();
            let operator = self.previous();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }
        expr
    }

    fn comparison(&mut self) -> Box<dyn Expr> {
        let mut expr: Box<dyn Expr> = self.term();
        let comparison_vec = vec![
            TokenType::Greater,
            TokenType::Greaterequal,
            TokenType::Less,
            TokenType::Lessequal,
        ];
        while self.matches(&comparison_vec) {
            let right = self.term();
            let operator = self.previous();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }
        expr
    }

    fn term(&mut self) -> Box<dyn Expr> {
        let mut expr: Box<dyn Expr> = self.factor();
        let term_vec = vec![TokenType::Minus, TokenType::Plus];
        while self.matches(&term_vec) {
            let right = self.factor();
            let operator = self.previous();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }
        expr
    }

    fn factor(&mut self) -> Box<dyn Expr> {
        let mut expr: Box<dyn Expr> = self.unary();
        let factor_vec = vec![TokenType::Star, TokenType::Slash];
        while self.matches(&factor_vec) {
            let right = self.unary();
            let operator = self.previous();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }
        expr
    }

    fn unary(&mut self) -> Box<dyn Expr> {
        let unary_vec = vec![TokenType::Bang, TokenType::Minus];
        if self.matches(&unary_vec) {
            let right = self.unary();
            let operator = self.previous();
            return Box::new(Unary::new(operator.clone(), right));
        }
        // TODO This should be looked at in the future unwrapping and letting program panic VS handling errors
        self.primary().unwrap()
    }

    fn primary(&mut self) -> Result<Box<dyn Expr>, io::Error> {
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
            // TODO unwrapping might have to be replaced with match or a different type of check.
            self.consume(TokenType::RightParen, "Expect ')' after expression.")
                .unwrap();
            return Ok(Box::new(Grouping::new(expr)));
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

    fn parse_error(&self, token: &Token, message: &str) -> io::Error {
        token_error(token, message);
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
        &self.tokens[self.current - 1]
    }
}
