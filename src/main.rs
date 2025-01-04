use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::Command;

const MAX_PROG_SIZE: usize = 30_000;

fn check_bounds(ptr: usize, array: &[u8]) -> Result<(), String> {
    if ptr >= array.len() {
        Err("Memory access out of bounds".to_string())
    } else {
        Ok(())
    }
}

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

fn interpret_bf(bf_code: &str) -> io::Result<()> {
    let mut array = [0u8; MAX_PROG_SIZE];
    let mut ptr: usize = 0;
    let mut code_ptr = 0;
    let mut loop_stack = Vec::new();

    while code_ptr < bf_code.len() {
        check_bounds(ptr, &array).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        match bf_code.chars().nth(code_ptr).unwrap() {
            '>' => ptr += 1,
            '<' => {
                ptr = ptr.checked_sub(1).ok_or(io::Error::new(
                    io::ErrorKind::Other,
                    "Memory access out of bounds",
                ))?
            }
            '+' => array[ptr] = array[ptr].wrapping_add(1),
            '-' => array[ptr] = array[ptr].wrapping_sub(1),
            '.' => print!("{}", array[ptr] as char),
            ',' => {
                let mut input = [0u8; 1];
                io::stdin().read_exact(&mut input)?;
                array[ptr] = input[0];
            }
            '[' => {
                if array[ptr] != 0 {
                    loop_stack.push(code_ptr);
                } else {
                    let mut depth = 1;
                    while depth > 0 {
                        code_ptr += 1;
                        match bf_code.chars().nth(code_ptr) {
                            Some('[') => depth += 1,
                            Some(']') => depth -= 1,
                            None => {
                                return Err(io::Error::new(
                                    io::ErrorKind::Other,
                                    "Mismatched brackets",
                                ))
                            }
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if array[ptr] != 0 {
                    code_ptr = *loop_stack
                        .last()
                        .ok_or(io::Error::new(io::ErrorKind::Other, "Mismatched brackets"))?;
                } else {
                    loop_stack.pop();
                }
            }
            _ => {}
        }
        code_ptr += 1;
    }
    Ok(())
}

fn repl_bf() -> io::Result<()> {
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

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mode = match args.get(1) {
        Some(mode) => mode,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No mode specified",
            ))
        }
    };
    let bf_path = match args.get(2) {
        Some(path) => Path::new(path),
        None => {
            if mode == "repl" {
                Path::new("")
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "No file specified",
                ));
            }
        }
    };

    match mode.as_str() {
        "build" => {
            let bf_code = read_bf_file(bf_path)?;
            let rust_code = compile_bf(&bf_code);

            match File::create("output.rs")?.write_all(rust_code.as_bytes()) {
                Ok(_) => {
                    let status = Command::new("rustc")
                        .arg("output.rs")
                        .arg("-o")
                        .arg("output")
                        .status()?;
                    if status.success() {
                        let save_output =
                            args.len() > 3 && (args[3] == "-s" || args[3] == "--save");
                        if !save_output {
                            std::fs::remove_file("output.rs")?;
                        }
                    } else {
                        return Err(io::Error::new(io::ErrorKind::Other, "Compilation failed"));
                    }
                }
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::ReadOnlyFilesystem, e));
                }
            }
        }
        "run" => {
            let bf_code = read_bf_file(bf_path)?;
            interpret_bf(&bf_code)?;
        }
        "repl" => repl_bf()?,
        _ => {
            eprintln!("Invalid mode: {}", mode);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode"));
        }
    }

    Ok(())
}

fn read_bf_file(bf_path: &Path) -> io::Result<String> {
    let mut bf_file = File::open(bf_path)?;
    let mut bf_code = String::new();
    bf_file.read_to_string(&mut bf_code)?;
    Ok(bf_code)
}
