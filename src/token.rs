use core::fmt::Debug;
use std::rc::Rc;
use strum_macros::Display;

use crate::{
    class::{LoxClass, LoxInstance},
    function::{LoxFunction, LoxNative},
};

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
    Nil,
    Function(LoxFunction),
    Native(LoxNative),
    Class(LoxClass),
    Instance(Rc<LoxInstance>),
}
impl DataType {
    pub fn to_string(&self) -> String {
        match self {
            DataType::String(s) => s.clone(),
            DataType::Number(n) => n.to_string(),
            DataType::Bool(b) => b.to_string(),
            DataType::Nil => "nil".to_string(),
            DataType::Function(f) => format!("{f}"),
            DataType::Native(n) => format!("{n}"),
            DataType::Class(c) => format!("{c}"),
            DataType::Instance(i) => format!("{i}"),
        }
    }
}
