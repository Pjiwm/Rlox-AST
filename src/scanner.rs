use crate::{
    error,
    token::{Token, TokenType}};
use std::fmt::Debug;
use substring::Substring;

pub struct Scanner<T: Debug> {
    source: String,
    tokens: Vec<Token<T>>,
    start: usize,
    current: usize,
    line: u32,
}
impl<T: Debug> Scanner<T> {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::<Token<T>>::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token<T>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            _ => error::error(self.line, "Unexpected character."),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current -1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_helper(token_type, None);
    }

    fn add_token_helper(&mut self, token_type: TokenType, literal: Option<T>) {
        let text = self.source.substring(self.start, self.current).to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }
}
