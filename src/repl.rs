use std::io::{self, Write};

use crate::interpreter::interpret_bf;

pub fn repl_bf() -> io::Result<()> {
    println!("Brainfuck REPL. Type 'exit' to quit.");
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "exit" {
            break;
        }

        interpret_bf(input.trim())?;
    }
    Ok(())
}
