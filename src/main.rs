use std::{env, process};

use colored::Colorize;

mod ast;
mod ast_printer;
mod class;
mod environment;
mod error;
mod function;
mod interpreter;
mod native_functions;
mod parser;
mod repl;
mod resolver;
mod run;
mod scanner;
mod source;
mod token;
#[macro_use]
extern crate lazy_static;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match source::run_file(args[1].as_str()) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", format!("{}", e).red());
                process::exit(65);
            }
        },
        1 | 0 => repl::prompt(),
        _ => {
            println!("Usage: jlox [script]");
            process::exit(64);
        }
    }
}
