use std::{
    io::{Error, ErrorKind},
    rc::Rc,
};

use crate::{error, interpreter::Interpreter, parser, resolver::Resolver, scanner};

pub fn run(source: &str, is_repl: bool) -> Result<(), Error> {
    let mut token_scanner = scanner::Scanner::new(source.to_string());
    let tokens = token_scanner.scan_tokens();
    let mut parser = parser::Parser::new(&tokens);
    let statements = match parser.parse() {
        Ok(expr) => expr,
        Err(e) => {
            return Err(e);
        }
    };
    let mut interpreter = Interpreter::new(is_repl);
    let mut resolver = Resolver::new(&interpreter);

    resolver.resolve(&Rc::new(statements.clone()));
    if error::get_resolve_error() {
        let e = Error::new(ErrorKind::Other, "Resolve error");
        return Err(e);
    }
    interpreter.interpret(statements);
    if error::get_runtime_error() {
        let e = Error::new(ErrorKind::Other, "Runtime error");
        return Err(e);
    }
    Ok(())
}

pub fn disable_errors() {
    error::set_error(false);
    error::set_runtime_error(false);
    error::set_resolve_error(false);
}
