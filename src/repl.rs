use crate::run;
use colored::Colorize;
use dialoguer::{console::style, theme::Theme, History, Input};
use std::{collections::VecDeque, process};
use substring::Substring;

pub fn prompt() {
    let mut history = MyHistory::default();
    clear();
    welcome();
    let mut input = String::new();
    loop {
        if let Ok(cmd) = Input::<String>::with_theme(&MyTheme::new())
            .history_with(&mut history)
            .interact_text()
        {
            if cmd == "exit" {
                process::exit(0);
            } else if cmd == "clear" {
                clear();
            } else if cmd == "reset" {
                input.clear();
            } else {
                input.push_str(&cmd);
                match run::run(&input, true) {
                    Ok(_) => {
                        input = remove_last_cmd(&input, &cmd);
                        let cmd = remove_print(&cmd);
                        input.push_str(cmd.as_str());
                    }
                    Err(_) => {
                        input = remove_last_cmd(&input, &cmd);
                        run::disable_errors();
                    }
                }
            }
        }
    }
}

fn clear() {
    match std::process::Command::new("cls").status() {
        Ok(_) => (),
        Err(_) => match std::process::Command::new("clear").status() {
            Ok(_) => (),
            Err(_) => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
        },
    }
}

fn welcome() {
    let version = env!("CARGO_PKG_VERSION").to_string().green();
    let author = env!("CARGO_PKG_AUTHORS").to_string().green();
    let green_arrow = ">>>".to_string().green();
    println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
    "================================================================================================".yellow(),
    format!("Lox Version: {version}").yellow(),
    format!("Author: {author}").yellow(),
    "A Rust implementation of the Lox language from the book Crafting Interpreters by Robert Nystorm.\n".yellow(),
    "Running in REPL mode. Type 'exit' to exit.".yellow(),
    "Other available commands:".yellow(),
    format!("{green_arrow} clear {}", "- Clears the terminal screen.".to_string().yellow() ),
    format!("{green_arrow} reset {}", "- Resets the input buffer.".to_string().yellow() ),
    format!("{green_arrow} exit {}", "- Exits the REPL.".to_string().yellow() ),
    "================================================================================================".yellow());
}

fn remove_last_cmd(input: &String, cmd: &str) -> String {
    input.substring(0, input.len() - cmd.len()).to_string()
}

fn remove_print(cmd: &str) -> String {
    let mut buffer = String::new();
    let mut in_print = false;
    for i in 0..cmd.len() {
        let current = cmd.substring(i, cmd.len() - 1);
        if current.starts_with("print") {
            in_print = true;
        }

        if !in_print {
            buffer.push(cmd.chars().nth(i).unwrap());
        }

        if current.starts_with(';') {
            in_print = false;
        }
    }
    buffer
}

struct MyHistory {
    max: usize,
    history: VecDeque<String>,
}

impl Default for MyHistory {
    fn default() -> Self {
        MyHistory {
            max: 4,
            history: VecDeque::new(),
        }
    }
}

impl<T: ToString> History<T> for MyHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        if self.history.len() == self.max {
            self.history.pop_back();
        }
        self.history.push_front(val.to_string());
    }
}

struct MyTheme {
    prefix: String,
    after_exec_prefix: String,
}

impl MyTheme {
    fn new() -> Self {
        MyTheme {
            prefix: style(">>> ").for_stderr().cyan().to_string(),
            after_exec_prefix: style(">>> ").for_stderr().green().to_string(),
        }
    }
}
impl Theme for MyTheme {
    fn format_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        write!(f, "{}{}", prompt, self.after_exec_prefix)
    }

    fn format_error(&self, f: &mut dyn std::fmt::Write, err: &str) -> std::fmt::Result {
        write!(f, "error: {}", err)
    }

    fn format_confirm_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> std::fmt::Result {
        if !prompt.is_empty() {
            write!(f, "{} ", &prompt)?;
        }
        match default {
            None => write!(f, "[y/n] ")?,
            Some(true) => write!(f, "[Y/n] ")?,
            Some(false) => write!(f, "[y/N] ")?,
        }
        Ok(())
    }

    fn format_confirm_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selection: Option<bool>,
    ) -> std::fmt::Result {
        let selection = selection.map(|b| if b { "yes" } else { "no" });

        match selection {
            Some(selection) if prompt.is_empty() => {
                write!(f, "{}", selection)
            }
            Some(selection) => {
                write!(f, "{} {}", &prompt, selection)
            }
            None if prompt.is_empty() => Ok(()),
            None => {
                write!(f, "{}", &prompt)
            }
        }
    }

    fn format_input_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> std::fmt::Result {
        match default {
            Some(default) if prompt.is_empty() => write!(f, "[{}]: ", default),
            Some(default) => write!(f, "{} [{}]: ", prompt, default),
            None => write!(f, "{}{} ", prompt, self.prefix),
        }
    }

    fn format_input_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        write!(f, "{}{} {}", prompt, self.after_exec_prefix, sel)
    }

    fn format_password_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt(f, prompt, None)
    }

    fn format_password_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt_selection(f, prompt, "[hidden]")
    }

    fn format_select_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_select_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> std::fmt::Result {
        self.format_input_prompt_selection(f, prompt, sel)
    }

    fn format_multi_select_prompt(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
    ) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_sort_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        self.format_prompt(f, prompt)
    }

    fn format_multi_select_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        write!(f, "{}> ", prompt)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(f, "{}{}", if idx == 0 { "" } else { ", " }, sel)?;
        }
        Ok(())
    }

    fn format_sort_prompt_selection(
        &self,
        f: &mut dyn std::fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> std::fmt::Result {
        self.format_multi_select_prompt_selection(f, prompt, selections)
    }

    fn format_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        active: bool,
    ) -> std::fmt::Result {
        write!(f, "{} {}", if active { ">" } else { " " }, text)
    }

    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            match (checked, active) {
                (true, true) => "> [x]",
                (true, false) => "  [x]",
                (false, true) => "> [ ]",
                (false, false) => "  [ ]",
            },
            text
        )
    }

    fn format_sort_prompt_item(
        &self,
        f: &mut dyn std::fmt::Write,
        text: &str,
        picked: bool,
        active: bool,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            match (picked, active) {
                (true, true) => "> [x]",
                (false, true) => "> [ ]",
                (_, false) => "  [ ]",
            },
            text
        )
    }
}
