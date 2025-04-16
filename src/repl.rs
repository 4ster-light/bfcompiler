use crate::interpreter::interpret_bf;
use colored::Colorize;
use std::io::{self, Write};

pub fn repl_bf() -> anyhow::Result<()> {
    println!("{}", "Brainfuck REPL. Type 'exit' to quit.".blue().bold());
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "exit" {
            break;
        }
        if !input.is_empty() {
            interpret_bf(input)?;
        }
    }
    Ok(())
}
