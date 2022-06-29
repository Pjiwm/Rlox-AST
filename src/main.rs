use std::{
    env,
    fs::File,
    io::{self, BufReader, Read, Write},
    process, str,
};
mod error;
mod scanner;
mod token;
mod expr;
#[macro_use]
extern crate lazy_static;

fn main() {
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
