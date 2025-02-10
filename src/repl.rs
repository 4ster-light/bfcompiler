use crate::interpreter::interpret_bf;
use colored::Colorize;
use std::io::{self, Write};

pub fn repl_bf() -> io::Result<()> {
    println!("{}", "Brainfuck REPL. Type 'exit' to quit.".blue().bold());
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        if input.trim() == "exit" {
            break;
        }
        if let Err(e) = interpret_bf(input.trim()) {
            eprintln!("{}: {}", "Error".red(), e);
        }
    }
    Ok(())
}
