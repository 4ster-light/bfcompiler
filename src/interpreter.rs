use crate::utils::MAX_PROG_SIZE;
use colored::Colorize;
use std::io::{self, Read};

pub fn interpret_bf(bf_code: &str) -> io::Result<()> {
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
                    "Memory access out of bounds".red().to_string(),
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
                                    "Mismatched brackets".red().to_string(),
                                ))
                            }
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if array[ptr] != 0 {
                    code_ptr = *loop_stack.last().ok_or(io::Error::new(
                        io::ErrorKind::Other,
                        "Mismatched brackets".red().to_string(),
                    ))?;
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

fn check_bounds(ptr: usize, array: &[u8]) -> Result<(), String> {
    if ptr >= array.len() {
        Err("Memory access out of bounds".red().to_string())
    } else {
        Ok(())
    }
}
