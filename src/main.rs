#![allow(incomplete_features)]
#![feature(unsized_locals, unsized_fn_params)]
use std::{
    env,
    io::{self, Write},
    process, str,
};

use expr::{Binary, Grouping, Literal, Unary};
use token::{DataType, Token, TokenType};
mod ast_printer;
mod error;
mod expr;
mod scanner;
mod token;
#[macro_use]
extern crate lazy_static;
extern crate dyn_safe;

fn main() {
    demo_ast();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[1].as_str()).unwrap();
        println!("SRC: {:?}", args[1].as_str());
    } else {
        run_prompt().unwrap();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let src = std::fs::read_to_string(path)?;
    print!("{}", src);
    run(&src).unwrap();
    if error::get_error() {
        process::exit(65);
    }
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();
    loop {
        print!("> ");
        stdout.flush()?;
        line.clear();
        stdin.read_line(&mut line)?;
        if line.is_empty() {
            break;
        }
        run(&line).unwrap();
        error::set_error(false);
    }
    Ok(())
}

fn run(source: &str) -> io::Result<()> {
    let mut token_scanner = scanner::Scanner::new(source.to_string());
    let tokens = token_scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn demo_ast() {
    let mut expression = Binary::new(
        Box::new(Unary::new(
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Box::new(Literal::new(DataType::Number(123.0))),
        )),
        Token::new(TokenType::Star, "*".to_string(), None, 1),
        Box::new(Grouping::new(Box::new(Literal::new(DataType::Number(
            45.67,
        ))))),
    );
    let expression = &mut expression;
    let mut printer = ast_printer::AstPrinter::new();
    let expression_str  = printer.print::<Binary>(expression);
    println!("{}", expression_str);
}
