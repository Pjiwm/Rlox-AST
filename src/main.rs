use std::{env, io, process, rc::Rc, str};

use ast::{Binary, Grouping, Literal, Unary};

use colored::Colorize;
use resolver::Resolver;
use token::{DataType, Token, TokenType};

use crate::interpreter::Interpreter;
mod ast;
mod ast_printer;
mod environment;
mod error;
mod function;
mod interpreter;
mod native_functions;
mod parser;
mod repl;
mod resolver;
mod scanner;
mod token;
#[macro_use]
extern crate lazy_static;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[1].as_str()).unwrap();
        println!("{}", format!("{} {}", "SRC:".yellow(), args[1].green()));
    } else {
        run_prompt().unwrap();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let src = std::fs::read_to_string(path)?;
    run(&src, false).unwrap();
    if error::get_error() {
        process::exit(65);
    }
    if error::get_runtime_error() {
        process::exit(70);
    }
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    repl::prompt();
    Ok(())
}

pub fn run(source: &str, is_repl: bool) -> io::Result<()> {
    let mut token_scanner = scanner::Scanner::new(source.to_string());
    let tokens = token_scanner.scan_tokens();
    let mut parser = parser::Parser::new(&tokens);
    // If we have an error during parsing, we want to print it and exit.
    let statements = match parser.parse() {
        Ok(expr) => expr,
        Err(_) => {
            return Ok(());
        }
    };
    let mut interpreter = Interpreter::new(is_repl);
    let mut resolver = Resolver::new(&interpreter);
    resolver.resolve(&Rc::new(statements.clone()));
    interpreter.interpret(statements);

    Ok(())
}

fn _demo_ast() {
    let expression = _binary_expression_sum();
    let mut printer = ast_printer::AstPrinter::_new();
    let expression_str = printer._print(Rc::new(expression));
    println!("{}", expression_str);
}

fn _binary_expression_multi() -> Binary {
    Binary::new(
        Rc::new(Unary::new(
            Token::new(TokenType::Minus, "-".to_string(), None, 1, 1),
            Rc::new(Literal::new(Some(DataType::Number(123.0)))),
        )),
        Token::new(TokenType::Star, "*".to_string(), None, 1, 2),
        Rc::new(Grouping::new(Rc::new(Literal::new(Some(
            DataType::Number(45.67),
        ))))),
    )
}

fn _binary_expression_sum() -> Binary {
    Binary::new(
        Rc::new(Literal::new(Some(DataType::Number(1.0)))),
        Token::new(TokenType::Plus, "+".to_string(), None, 1, 3),
        Rc::new(Literal::new(Some(DataType::Number(2.0)))),
    )
}
