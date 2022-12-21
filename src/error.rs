use crate::token::Token;
use colored::*;

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;
static mut HAD_RESOLVE_ERROR: bool = false;

pub fn error(line: u32, pos: u32, message: &str) {
    report(line, pos, message);
}

pub fn token_error(token: &Token, message: &str) {
    report(token.line, token.pos, message);
}

pub fn runtime_error(token: &Option<Token>, message: &str) {
    let message = format!("[Runtime error] {}", message.to_owned());
    unsafe {
        HAD_RUNTIME_ERROR = true;
    }
    match token {
        Some(t) => report(t.line, t.pos, message.as_str()),
        None => report(0, 0, message.as_str()),
    }
}

pub fn parse_error(token: &Token, message: &str) {
    let message = format!("[Parse error] {}", message.to_owned());
    token_error(token, message.as_str());
}

pub fn resolve_error(token: &Token, message: &str) {
    let message = format!("[Resolve error] {}", message.to_owned());
    token_error(token, message.as_str());
    unsafe {
        HAD_RESOLVE_ERROR = true;
    }
}

fn report(line: u32, pos: u32, message: &str) {
    if line == 0 {
        println!("{}", format!("Error: {message}").red());
    } else {
        println!("{}", format!("Error at line {line}-{pos}: {message}").red());
    }
    set_error(true);
}

pub fn set_error(error: bool) {
    unsafe {
        HAD_ERROR = error;
    }
}

pub fn set_runtime_error(error: bool) {
    unsafe {
        HAD_RUNTIME_ERROR = error;
    }
}

pub fn set_resolve_error(error: bool) {
    unsafe {
        HAD_RESOLVE_ERROR = error;
    }
}

pub fn get_error() -> bool {
    unsafe { HAD_ERROR }
}

pub fn get_runtime_error() -> bool {
    unsafe { HAD_RUNTIME_ERROR }
}

pub fn get_resolve_error() -> bool {
    unsafe { HAD_RESOLVE_ERROR }
}
