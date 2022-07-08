// TODO fix borrowing and ownership in parser
use crate::{
    expr::{Binary, Expr, Unary, Literal, Grouping},
    token::{Token, TokenType, DataType},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&self) -> Box<dyn Expr> {
        self.equality()
    }

    fn equality(&self) -> Box<dyn Expr> {
        let expr: Box<dyn Expr> = self.comparison();
        let equal_vec = vec![TokenType::Equalequal, TokenType::Bangequal];
        while self.matches(equal_vec) {
            let operator = self.previous();
            let right = self.comparison();
            let expr = Binary::new(expr, operator.clone(), right);
        }
        expr
    }

    fn comparison(&self) -> Box<dyn Expr> {
        let expr: Box<dyn Expr> = self.term();
        let comparison_vec = vec![
            TokenType::Greater,
            TokenType::Greaterequal,
            TokenType::Less,
            TokenType::Lessequal,
        ];
        while self.matches(comparison_vec) {
            let operator = self.previous();
            let right = self.term();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }
        expr
    }

    fn term(&self) -> Box<dyn Expr> {
        let expr: Box<dyn Expr> = self.factor();
        let term_vec = vec![TokenType::Minus, TokenType::Plus];
        while self.matches(term_vec) {
            let operator = self.previous();
            let right = self.factor();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }
        expr
    }

    fn factor(&self) -> Box<dyn Expr> {
        let expr: Box<dyn Expr> = self.unary();
        let factor_vec = vec![TokenType::Star, TokenType::Slash];
        while self.matches(factor_vec) {
            let operator = self.previous();
            let right = self.unary();
            expr = Box::new(Binary::new(expr, operator.clone(), right));
        }
        expr
    }

    fn unary(&self) -> Box<dyn Expr> {
        let unary_vec = vec![TokenType::Bang, TokenType::Minus];
        if self.matches(unary_vec) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(Unary::new(operator.clone(), right));
        }
        self.primary()
    }

    fn primary(&self) -> Box<dyn Expr> {
        let false_vec = vec![TokenType::False];
        if self.matches(false_vec) {
            return Box::new(Literal::new(Some(DataType::False(false))));
        } 
        let true_vec = vec![TokenType::True];
        if self.matches(true_vec) {
            return Box::new(Literal::new(Some(DataType::True(true))));
        }
        let nil_vec = vec![TokenType::Nil];
        if self.matches(nil_vec) {
            return Box::new(Literal::new(None));
        }

        let data_type_vec = vec![TokenType::Number, TokenType::String];
        if self.matches(data_type_vec) {
            let data_type = self.previous();
            return Box::new(Literal::new(Some(data_type.literal.unwrap())));
        }

        let left_paren_vec = vec![TokenType::LeftParen];
        if self.matches(left_paren_vec) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Box::new(Grouping::new(expr));
        }
        // If none of the above apply we'll jut return Nil
        Box::new(Literal::new(None))
    }

    // TODO implement consume
    fn consume(&self, token_type: TokenType, message: &str) {
        if self.check(token_type) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }

    fn matches(&self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }

    fn advance(&self) -> &Token {
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
