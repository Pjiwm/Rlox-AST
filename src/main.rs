use std::{
    env,
    fs::File,
    io::{self, BufReader, Read, Write},
    process, str,
};
mod error;
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
    } else {
        run_prompt().unwrap();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut bytes = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut bytes)?;
    run(str::from_utf8(&bytes).unwrap()).unwrap();
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
    let mut token_scanner = scanner::Scanner::new(source.replace("\n", ""));
    let tokens = token_scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
