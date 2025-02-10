use crate::utils::{read_bf_file, MAX_PROG_SIZE};
use colored::Colorize;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn compile_bf(bf_code: &str) -> String {
    let mut output = String::new();
    output.push_str("fn main() -> Result<(), Box<dyn std::error::Error>> {let mut array = [0u8; ");
    output.push_str(&MAX_PROG_SIZE.to_string());
    output.push_str("];let mut ptr: usize = 0;");
    let mut bracket_count = 0;
    for ch in bf_code.chars() {
        match ch {
            '>' => output.push_str("ptr += 1; check_bounds(ptr, &array)?;"),
            '<' => output.push_str("ptr = ptr.checked_sub(1).ok_or(\"Memory access out of bounds\")?; check_bounds(ptr, &array)?;"),
            '+' => output.push_str("array[ptr] = array[ptr].wrapping_add(1);"),
            '-' => output.push_str("array[ptr] = array[ptr].wrapping_sub(1);"),
            '.' => output.push_str("print!(\"{}\", array[ptr] as char);"),
            ',' => output.push_str("array[ptr] = io::stdin().bytes().next().and_then(|r| r.ok()).unwrap_or(0);"),
            '[' => {
                bracket_count += 1;
                output.push_str("while array[ptr] != 0 {");
            },
            ']' => {
                if bracket_count > 0 {
                    output.push_str("}");
                    bracket_count -= 1;
                }
            },
            _ => {}
        }
    }
    output.push_str("Ok(())}");
    output.push_str("fn check_bounds(ptr: usize, _array: &[u8]) -> Result<(), String> {if ptr >= ");
    output.push_str(&MAX_PROG_SIZE.to_string());
    output.push_str(" { Err(\"Memory access out of bounds\".to_string()) } else { Ok(()) }}");
    output
}

pub fn build_bf(bf_path: PathBuf, save_output: bool) -> io::Result<()> {
    let mut output_file = File::create("output.rs")?;
    output_file.write_all(compile_bf(&read_bf_file(&bf_path)?).as_bytes())?;

    let status = Command::new("rustc")
        .arg("output.rs")
        .arg("-o")
        .arg("output")
        .status()?;

    if status.success() {
        if !save_output {
            std::fs::remove_file("output.rs")?;
        }
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Compilation failed".red().to_string(),
        ))
    }
}
