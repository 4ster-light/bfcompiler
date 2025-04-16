use crate::error::Error;
use crate::utils::{read_bf_file, MAX_PROG_SIZE};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn compile_bf(bf_code: &str) -> anyhow::Result<String> {
    let mut output = String::new();

    let mut needs_io = false;
    let mut needs_mut_array = false;
    let mut needs_mut_ptr = false;
    let mut needs_bounds_check = false;

    for ch in bf_code.chars() {
        match ch {
            '+' | '-' => needs_mut_array = true,
            ',' => {
                needs_mut_array = true;
                needs_io = true;
            }
            '>' | '<' => {
                needs_mut_ptr = true;
                needs_bounds_check = true;
            }
            '.' => needs_io = true,
            _ => {}
        }
    }

    if needs_io {
        output.push_str("use std::io;\n");
    }
    output.push_str("fn main() -> Result<(), Box<dyn std::error::Error>> {");

    output.push_str(&format!(
        "let {}array = [0u8; {}];",
        if needs_mut_array { "mut " } else { "" },
        MAX_PROG_SIZE
    ));
    output.push_str(&format!(
        "let {}ptr: usize = 0;",
        if needs_mut_ptr { "mut " } else { "" }
    ));

    let mut bracket_count = 0;
    for (i, ch) in bf_code.chars().enumerate() {
        match ch {
            '>' => output.push_str("ptr += 1;"),
            '<' => {
                output.push_str("ptr = ptr.checked_sub(1).ok_or(\"Memory access out of bounds\")?;")
            }
            '+' => output.push_str("array[ptr] = array[ptr].wrapping_add(1);"),
            '-' => output.push_str("array[ptr] = array[ptr].wrapping_sub(1);"),
            '.' => output.push_str("print!(\"{}\", array[ptr] as char);"),
            ',' => output.push_str(
                "array[ptr] = io::stdin().bytes().next().and_then(|r| r.ok()).unwrap_or(0);",
            ),
            '[' => {
                bracket_count += 1;
                output.push_str("while array[ptr] != 0 {");
            }
            ']' => {
                if bracket_count > 0 {
                    output.push_str("}");
                    bracket_count -= 1;
                } else {
                    return Err(Error::MismatchedBrackets(i).into());
                }
            }
            _ => {}
        }
        if matches!(ch, '>' | '<') && needs_bounds_check {
            output.push_str("check_bounds(ptr, &array)?;");
        }
    }
    if bracket_count != 0 {
        return Err(Error::MismatchedBrackets(bf_code.len()).into());
    }
    output.push_str("println!(\"\");");
    output.push_str("Ok(())}");

    if needs_bounds_check {
        output.push_str(&format!(
            "fn check_bounds(ptr: usize, _array: &[u8]) -> Result<(), String> {{if ptr >= {} {{ Err(\"Memory access out of bounds\".to_string()) }} else {{ Ok(()) }}}}",
            MAX_PROG_SIZE
        ));
    }

    Ok(output)
}

pub fn build_bf(bf_path: PathBuf, save_output: bool) -> anyhow::Result<()> {
    let bf_code = read_bf_file(&bf_path)?;
    eprintln!("Building BF file: {:?}\nCode: {}", bf_path, bf_code);
    let compiled_code = compile_bf(&bf_code)?;
    let output_rs = bf_path.with_extension("rs");
    let mut output_file = File::create(&output_rs)?;
    output_file.write_all(compiled_code.as_bytes())?;

    let output_bin = bf_path.with_extension("");
    let status = Command::new("rustc")
        .arg(&output_rs)
        .arg("-o")
        .arg(&output_bin)
        .stderr(std::process::Stdio::piped())
        .output()?;

    if status.status.success() {
        if !save_output {
            std::fs::remove_file(&output_rs)?;
        }
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&status.stderr);
        Err(Error::CompilationFailed(format!("Rust compilation failed: {}", stderr)).into())
    }
}
