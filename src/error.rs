use crate::token::{Token, TokenType};

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

pub fn error(line: u32, message: &str) {
    report(line, message);
}

pub fn token_error(token: &Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, message);
    } else {
        report(token.line, message);
    }
}

pub fn runtime_error(token: &Option<Token>, message: &str) {
    unsafe {
        HAD_RUNTIME_ERROR = true;
    }
    match token {
        Some(t) => report(t.line, message),
        None => report(0, message),
    }
}

fn report(line: u32, message: &str) {
    println!("Error at line {}: {}", line, message);
    println!("{}", message);
    set_error(true);
}

pub fn set_error(error: bool) {
    unsafe {
        HAD_ERROR = error;
    }
}

pub fn get_error() -> bool {
    unsafe { HAD_ERROR }
}
