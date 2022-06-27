use std::any::Any;

use crate::{
    error,
    token::{DataType, Token, TokenType},
};
use substring::Substring;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    /// Keeps looping in search of lexemes until the end of the source is reached.
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        return &self.tokens;
    }
    /// Scans the next token.
    fn scan_token(&mut self) {
        // self.advance(); gives the next token,
        // by incrementing the current index and returning the character at that index.
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
            '*' => self.add_token(TokenType::Star),
            // These lexemes can be either a one or two character long token.
            // The pick and add token fn, will handle this.
            '!' => self.pick_and_add_token(TokenType::Bangequal, TokenType::Bang, '='),
            '=' => self.pick_and_add_token(TokenType::Equalequal, TokenType::Equal, '='),
            '<' => self.pick_and_add_token(TokenType::Lessequal, TokenType::Less, '='),
            '>' => self.pick_and_add_token(TokenType::Greaterequal, TokenType::Greater, '='),
            '"' => self.string(),
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            // Ignore whitespace
            ' ' | '\r' | '\t' => (),
            // Indicates a new line, so the line in the struct should advance as well.
            '\n' => self.line += 1,
            _ => error::error(self.line, "Unexpected character."),
        }
    }

    /// Scans a string literal.
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error::error(self.line, "Unterminated string.");
            return;
        }
        self.advance();
        let value = self
            .source
            .substring(self.start + 1, self.current - 1)
            .to_string();
        self.add_token_helper(TokenType::String, Some(DataType::String(value)));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    /// Advance the current index by one. And returns the char at the current index.
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }
    /// Adds a new token to the tokens vector.
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_helper(token_type, None);
    }

    /// Gives back the correct token type for a lexeme.
    /// Used only for lexemes which are potentially two characters long.
    fn pick_and_add_token(&mut self, token_match: TokenType, token_mismatch: TokenType, c: char) {
        if self.matches(c) {
            self.add_token(token_match);
        } else {
            self.add_token(token_mismatch);
        }
    }

    fn add_token_helper(&mut self, token_type: TokenType, literal: Option<DataType>) {
        let text = self.source.substring(self.start, self.current).to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }
    /// Checks if the next char is the same as the given char. This is used to check for lexemes of two characters.
    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    /// Peeks the upcoming character without advancing the current index.
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }
}
