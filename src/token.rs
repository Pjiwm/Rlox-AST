use core::fmt::Debug;
use strum_macros::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<DataType>,
    pub line: u32,
}
impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<DataType>,
        line: u32,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
    pub fn dup(&self) -> Token {
        Token {
            token_type: self.token_type,
            lexeme: self.lexeme.clone(),
            literal: self.literal.clone(),
            line: self.line,
        }
    }
}
#[derive(Display, Clone, Debug, PartialEq, Copy)]
pub enum TokenType {
    // single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // one or two character tokens.
    Bang,
    Bangequal,
    Equal,
    Equalequal,
    Greater,
    Greaterequal,
    Less,
    Lessequal,
    // literals.
    Identifier,
    String,
    Number,
    // keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}
impl DataType {
    pub fn dup(&self) -> DataType {
        match self {
            DataType::String(s) => DataType::String(s.clone()),
            DataType::Number(n) => DataType::Number(*n),
            DataType::Bool(b) => DataType::Bool(*b),
            DataType::Nil => DataType::Nil,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DataType::String(s) => s.clone(),
            DataType::Number(n) => n.to_string(),
            DataType::Bool(b) => b.to_string(),
            DataType::Nil => "nil".to_string(),
        }
    }
}
