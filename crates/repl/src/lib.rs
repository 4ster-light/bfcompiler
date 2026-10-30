use anyhow::Result;
use bf_interpreter::{execute, DEFAULT_STEP_LIMIT};
use colored::Colorize;
use std::io::{self, Write};

pub fn repl_bf() -> Result<()> {
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
            match execute(input, &[], DEFAULT_STEP_LIMIT) {
                Ok(execution) => {
                    io::stdout().write_all(&execution.output)?;
                    println!();
                }
                Err(error) => eprintln!("{} {error}", "Error:".red().bold()),
            }
        }
    }
    Ok(())
}
