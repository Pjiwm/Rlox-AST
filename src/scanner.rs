use crate::{
    error,
    token::{DataType, Token, TokenType},
};
use std::collections::HashMap;
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
                // This is for single line comments
                if self.matches('/') {
                    while (self.peek() != '\n') && !self.is_at_end() {
                        self.advance();
                    }
                // This is for multi line comments
                } else if self.matches('*') {
                    while !self.is_at_end() {
                        // Matching with new lines is important to keep the line property up-to-date.
                        if self.matches('\n') {
                            self.line += 1;
                        }
                        self.advance();
                        // A multiline comment always ends with a */, so we can check for that.
                        if self.matches('/') {
                            break;
                        }
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            } /* */
            // Ignore whitespace
            ' ' | '\r' | '\t' => (),
            // Indicates a new line, so the line in the struct should advance as well.
            '\n' => self.line += 1,
            // Default case
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    error::error(self.line, "Unexpected character.")
                }
            }
        }
    }

    /// Checks if the lexeme is a keyword or an identifier and adds it to the tokens vector.
    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = self.source.substring(self.start, self.current);
        let token_type = KEYWORDS.get(text);
        // If the token is not a keyword it is always an identifier.
        match token_type {
            Some(token_type) => self.add_token(token_type.clone()),
            None => self.add_token(TokenType::Identifier),
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
        self.add_token_advanced(TokenType::String, Some(DataType::String(value)));
    }

    /// Scans a number literal.
    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        // Look for fractional part.
        // Looking past decimal point requires a second character of lookeahead
        // since we don't want to consume the . until we're  sure there's a digit after it.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        // Grab the substring containing just the number.
        let value = self.source.substring(self.start, self.current).to_string();
        // Wrap and convert it to the right datatype.
        let value = Some(DataType::Number(value.parse::<f64>().unwrap()));
        self.add_token_advanced(TokenType::Number, value);
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
        self.add_token_advanced(token_type, None);
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

    fn add_token_advanced(&mut self, token_type: TokenType, literal: Option<DataType>) {
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

    /// Peeks the 2nd upcoming character without advancing the current index.
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    /// Checks if the given char is a letter or _.
    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    /// Checks if value is digit
    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    /// Checks if the given char is a digit or a letter.
    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
}

lazy_static! {
    /// Statically define all keywords
    pub static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert(String::from("and"), TokenType::And);
        m.insert(String::from("class"), TokenType::Class);
        m.insert(String::from("else"), TokenType::Else);
        m.insert(String::from("false"), TokenType::False);
        m.insert(String::from("fun"), TokenType::Fun);
        m.insert(String::from("for"), TokenType::For);
        m.insert(String::from("if"), TokenType::If);
        m.insert(String::from("nil"), TokenType::Nil);
        m.insert(String::from("or"), TokenType::Or);
        m.insert(String::from("print"), TokenType::Print);
        m.insert(String::from("return"), TokenType::Return);
        m.insert(String::from("super"), TokenType::Super);
        m.insert(String::from("this"), TokenType::This);
        m.insert(String::from("true"), TokenType::True);
        m.insert(String::from("var"), TokenType::Var);
        m.insert(String::from("while"), TokenType::While);
        m
    };
}
