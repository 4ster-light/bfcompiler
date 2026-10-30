use anyhow::Result;
use bf_interpreter::{execute, DEFAULT_STEP_LIMIT};
use colored::Colorize;
use std::io::{self, Write};

/// Interpret a complete program using the process standard input and output.
pub fn interpret_bf(bf_code: &str) -> Result<()> {
    let input = if bf_code.contains(',') {
        io::read_to_string(io::stdin())?
    } else {
        String::new()
    };
    let execution = execute(bf_code, input.as_bytes(), DEFAULT_STEP_LIMIT)?;
    io::stdout().write_all(&execution.output)?;
    println!("{}", "Execution successful!".green().bold());
    Ok(())
}
