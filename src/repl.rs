use crate::interpreter::interpret_bf;
use std::io::{self, Write};

pub fn repl_bf() -> io::Result<()> {
    println!("Brainfuck REPL. Type 'exit' to quit.");
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        if input.trim() == "exit" {
            break;
        }
        interpret_bf(input.trim())?;
    }
    Ok(())
}
