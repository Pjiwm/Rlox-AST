use crate::token::{Token, TokenType};
use colored::*;

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
    let message = format!("[Runtime error] {}", message.to_owned());
    unsafe {
        HAD_RUNTIME_ERROR = true;
    }
    match token {
        Some(t) => report(t.line, message.as_str()),
        None => report(0, message.as_str()),
    }
}

pub fn parse_error(token: &Token, message: &str) {
    let message = format!("[Parse error] {}", message.to_owned());
    token_error(token, message.as_str());
}

fn report(line: u32, message: &str) {
    println!("{}", format!("Error at line {line}: {message}").red());
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

pub fn get_runtime_error() -> bool {
    unsafe { HAD_RUNTIME_ERROR }
}
