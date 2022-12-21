use strum_macros::Display;

use crate::{
    class::{LoxClass, LoxInstance},
    function::{LoxFunction, LoxNative},
};
use core::fmt::{Debug, Display};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<DataType>,
    pub line: u32,
    pub pos: u32,
}
impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<DataType>,
        line: u32,
        pos: u32,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            pos,
        }
    }
    pub fn dup(&self) -> Token {
        Token {
            token_type: self.token_type,
            lexeme: self.lexeme.clone(),
            literal: self.literal.clone(),
            line: self.line,
            pos: self.pos,
        }
    }
}
#[derive(Display, Clone, Debug, PartialEq, Copy, Hash)]
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
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
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
    Nil,
    Function(LoxFunction),
    Native(LoxNative),
    Class(LoxClass),
    Instance(Rc<LoxInstance>),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::String(s) => write!(f, "{s}"),
            DataType::Number(n) => write!(f, "{n}"),
            DataType::Bool(b) => write!(f, "{b}"),
            DataType::Nil => write!(f, "NIL"),
            DataType::Function(fnc) => write!(f, "{fnc}"),
            DataType::Native(n) => write!(f, "{n}"),
            DataType::Class(c) => write!(f, "{c}"),
            DataType::Instance(i) => write!(f, "{i}"),
        }
    }
}
