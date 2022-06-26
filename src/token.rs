use core::fmt::Debug;
#[derive(Clone, Debug)] 
pub struct Token<T: Debug> {
    token_type: TokenType,
    lexeme: String,
    literal: Option<T>,
    line: u32,
}
impl<T: Debug> Token<T> {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<T>, line: u32) -> Self {
        Token {
            token_type,
            lexeme,  
            literal,
            line,
        }
    }
    pub fn print(&self) -> String {
        format!(
            "type: {}, lexeme: {}, literal: {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
#[derive(strum_macros::Display, Clone, Debug)]
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
