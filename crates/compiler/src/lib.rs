use anyhow::Result;
use bf_interpreter::{Error as InterpreterError, Interpreter, TAPE_SIZE};
use colored::Colorize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Memory access out of bounds at cell {0}")]
    BoundsError(usize),
    #[error("Mismatched bracket at source position {0}")]
    MismatchedBrackets(usize),
    #[error("Compilation to Rust code failed: {0}")]
    CompilationFailed(String),
}

impl From<InterpreterError> for Error {
    fn from(error: InterpreterError) -> Self {
        match error {
            InterpreterError::BoundsError(pointer) => Self::BoundsError(pointer),
            InterpreterError::MismatchedBrackets(position) => Self::MismatchedBrackets(position),
            InterpreterError::StepLimitExceeded(_) => unreachable!("validation does not execute"),
        }
    }
}

/// Translate Brainfuck into a small, dependency-free Rust program.
pub fn compile_bf(bf_code: &str) -> Result<String, Error> {
    // Reuse the parser so the compiler and interpreter reject identical programs.
    Interpreter::new(bf_code, Vec::new())?;

    let mut output = String::new();
    let needs_input = bf_code.contains(',');
    let needs_output = bf_code.contains('.');
    let needs_mut_array = needs_input || bf_code.contains(['+', '-']);
    let needs_mut_ptr = bf_code.contains(['>', '<']);

    if needs_input || needs_output {
        output.push_str("use std::io::{self");
        if needs_input {
            output.push_str(", Read");
        }
        if needs_output {
            output.push_str(", Write");
        }
        output.push_str("};\n");
    }

    output.push_str("fn main() -> Result<(), Box<dyn std::error::Error>> {");
    output.push_str(&format!(
        "let {}array = [0u8; {}];",
        if needs_mut_array { "mut " } else { "" },
        TAPE_SIZE
    ));
    output.push_str(&format!(
        "let {}ptr: usize = 0;",
        if needs_mut_ptr { "mut " } else { "" }
    ));

    for ch in bf_code.chars() {
        match ch {
            '>' => output.push_str("ptr += 1;check_bounds(ptr, &array)?;"),
            '<' => {
                output.push_str("ptr = ptr.checked_sub(1).ok_or(\"Memory access out of bounds\")?;")
            }
            '+' => output.push_str("array[ptr] = array[ptr].wrapping_add(1);"),
            '-' => output.push_str("array[ptr] = array[ptr].wrapping_sub(1);"),
            '.' => output.push_str("io::stdout().write_all(&[array[ptr]])?;"),
            ',' => output.push_str(
                "array[ptr] = io::stdin().bytes().next().and_then(|r| r.ok()).unwrap_or(0);",
            ),
            '[' => output.push_str("while array[ptr] != 0 {"),
            ']' => output.push('}'),
            _ => {}
        }
    }

    output.push_str("Ok(())}");
    if needs_mut_ptr {
        output.push_str(&format!(
            "fn check_bounds(ptr: usize, array: &[u8]) -> Result<(), String> {{if ptr >= {} {{ Err(format!(\"Memory access out of bounds at cell {{}}\", ptr)) }} else {{ let _ = array; Ok(()) }}}}",
            TAPE_SIZE
        ));
    }
    Ok(output)
}

pub fn build_bf(bf_path: PathBuf, save_output: bool) -> Result<()> {
    let bf_code = std::fs::read_to_string(&bf_path)?;
    let compiled_code = compile_bf(&bf_code)?;
    let output_rs = bf_path.with_extension("rs");
    File::create(&output_rs)?.write_all(compiled_code.as_bytes())?;

    let output_bin = bf_path.with_extension("");
    let result = Command::new("rustc")
        .arg(&output_rs)
        .arg("-o")
        .arg(&output_bin)
        .stderr(std::process::Stdio::piped())
        .output()?;

    if result.status.success() {
        if !save_output {
            std::fs::remove_file(&output_rs)?;
        }
        println!("{}", "Build successful!".green().bold());
        Ok(())
    } else {
        Err(Error::CompilationFailed(format!(
            "Rust compilation failed: {}",
            String::from_utf8_lossy(&result.stderr)
        ))
        .into())
    }
}
