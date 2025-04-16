use crate::error::Error;
use crate::utils::MAX_PROG_SIZE;
use std::io::{self, Read};

pub fn interpret_bf(bf_code: &str) -> anyhow::Result<()> {
    let mut array = [0u8; MAX_PROG_SIZE];
    let mut ptr = 0;
    let mut code_ptr = 0;
    let mut loop_stack = Vec::new();

    while code_ptr < bf_code.len() {
        if ptr >= array.len() {
            return Err(Error::BoundsError(ptr).into());
        }

        match bf_code.chars().nth(code_ptr).unwrap() {
            '>' => ptr += 1,
            '<' => {
                ptr = ptr.checked_sub(1).ok_or_else(|| Error::BoundsError(ptr))?;
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
                            None => return Err(Error::MismatchedBrackets(code_ptr).into()),
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if array[ptr] != 0 {
                    code_ptr = *loop_stack
                        .last()
                        .ok_or(Error::MismatchedBrackets(code_ptr))?;
                } else {
                    loop_stack.pop();
                }
            }
            _ => {}
        }
        code_ptr += 1;
    }

    if !loop_stack.is_empty() {
        return Err(Error::MismatchedBrackets(code_ptr).into());
    }

    print!("\n");
    Ok(())
}
