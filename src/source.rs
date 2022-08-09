use std::{io, process};

use colored::Colorize;

use crate::{error, run};

pub fn run_file(path: &str) -> io::Result<()> {
    let src = std::fs::read_to_string(path)?;
    // run(&src).unwrap();
    match run::run(&src, false) {
        Ok(_) => {
            println!("{}", format!("{} {}", "SRC:".yellow(), path.green()));
        }
        Err(_) => {
            if error::get_error() {
                process::exit(65);
            } else if error::get_runtime_error() {
                process::exit(70);
            } else if error::get_resolve_error() {
                process::exit(71);
            } else {
                process::exit(0);
            }
        }
    }
    Ok(())
}
