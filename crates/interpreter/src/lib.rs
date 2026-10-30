//! Portable Brainfuck execution, independent of any terminal or browser APIs.

use thiserror::Error;

pub const TAPE_SIZE: usize = 30_000;
pub const DEFAULT_STEP_LIMIT: u64 = 10_000_000;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Memory access out of bounds at cell {0}")]
    BoundsError(usize),
    #[error("Mismatched bracket at source position {0}")]
    MismatchedBrackets(usize),
    #[error("Execution paused after {0} instructions; the step limit was reached")]
    StepLimitExceeded(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    Running,
    Finished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Execution {
    pub output: Vec<u8>,
    pub steps: u64,
    pub pointer: usize,
}

/// A resumable Brainfuck machine. Every call to `step` executes one instruction.
#[derive(Debug, Clone)]
pub struct Interpreter {
    program: Vec<u8>,
    brackets: Vec<usize>,
    tape: Vec<u8>,
    input: Vec<u8>,
    output: Vec<u8>,
    instruction_pointer: usize,
    input_pointer: usize,
    pointer: usize,
    steps: u64,
}

impl Interpreter {
    pub fn new(source: &str, input: impl Into<Vec<u8>>) -> Result<Self, Error> {
        let mut program = Vec::new();
        let mut positions = Vec::new();
        let mut open_brackets = Vec::new();

        for (position, byte) in source.bytes().enumerate() {
            if matches!(byte, b'>' | b'<' | b'+' | b'-' | b'.' | b',' | b'[' | b']') {
                program.push(byte);
                positions.push(position);
            }
        }

        let mut brackets = vec![usize::MAX; program.len()];
        for (index, &instruction) in program.iter().enumerate() {
            match instruction {
                b'[' => open_brackets.push(index),
                b']' => {
                    let open = open_brackets
                        .pop()
                        .ok_or(Error::MismatchedBrackets(positions[index]))?;
                    brackets[open] = index;
                    brackets[index] = open;
                }
                _ => {}
            }
        }

        if let Some(&open) = open_brackets.last() {
            return Err(Error::MismatchedBrackets(positions[open]));
        }

        Ok(Self {
            program,
            brackets,
            tape: vec![0; TAPE_SIZE],
            input: input.into(),
            output: Vec::new(),
            instruction_pointer: 0,
            input_pointer: 0,
            pointer: 0,
            steps: 0,
        })
    }

    pub fn step(&mut self) -> Result<RunStatus, Error> {
        let Some(&instruction) = self.program.get(self.instruction_pointer) else {
            return Ok(RunStatus::Finished);
        };

        match instruction {
            b'>' => {
                self.pointer += 1;
                if self.pointer >= self.tape.len() {
                    return Err(Error::BoundsError(self.pointer));
                }
            }
            b'<' => {
                self.pointer = self
                    .pointer
                    .checked_sub(1)
                    .ok_or(Error::BoundsError(self.pointer))?;
            }
            b'+' => self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1),
            b'-' => self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1),
            b'.' => self.output.push(self.tape[self.pointer]),
            b',' => {
                self.tape[self.pointer] = self.input.get(self.input_pointer).copied().unwrap_or(0);
                self.input_pointer = self.input_pointer.saturating_add(1);
            }
            b'[' if self.tape[self.pointer] == 0 => {
                self.instruction_pointer = self.brackets[self.instruction_pointer];
            }
            b']' if self.tape[self.pointer] != 0 => {
                self.instruction_pointer = self.brackets[self.instruction_pointer];
            }
            _ => {}
        }

        self.instruction_pointer += 1;
        self.steps += 1;
        Ok(if self.instruction_pointer == self.program.len() {
            RunStatus::Finished
        } else {
            RunStatus::Running
        })
    }

    /// Execute at most `max_steps` more instructions and retain the machine state.
    pub fn run(&mut self, max_steps: u64) -> Result<RunStatus, Error> {
        for _ in 0..max_steps {
            if self.step()? == RunStatus::Finished {
                return Ok(RunStatus::Finished);
            }
        }
        Ok(self.status())
    }

    pub fn status(&self) -> RunStatus {
        if self.instruction_pointer >= self.program.len() {
            RunStatus::Finished
        } else {
            RunStatus::Running
        }
    }

    pub fn output(&self) -> &[u8] {
        &self.output
    }

    pub fn tape(&self) -> &[u8] {
        &self.tape
    }

    pub fn pointer(&self) -> usize {
        self.pointer
    }

    pub fn instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    pub fn steps(&self) -> u64 {
        self.steps
    }
}

/// Run a complete program with a fixed instruction budget.
pub fn execute(source: &str, input: &[u8], max_steps: u64) -> Result<Execution, Error> {
    let mut interpreter = Interpreter::new(source, input.to_vec())?;
    if interpreter.run(max_steps)? == RunStatus::Running {
        return Err(Error::StepLimitExceeded(max_steps));
    }
    Ok(Execution {
        output: interpreter.output,
        steps: interpreter.steps,
        pointer: interpreter.pointer,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_hello_world() {
        let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let result = execute(source, &[], DEFAULT_STEP_LIMIT).unwrap();
        assert_eq!(result.output, b"Hello World!\n");
    }

    #[test]
    fn input_and_cells_wrap() {
        assert_eq!(execute(",.", b"A", 10).unwrap().output, b"A");
        assert_eq!(execute("-.", &[], 10).unwrap().output, [255]);
        assert_eq!(execute(",+.", &[], 10).unwrap().output, [1]);
    }

    #[test]
    fn rejects_invalid_programs_before_execution() {
        assert!(matches!(
            Interpreter::new("+[+", []),
            Err(Error::MismatchedBrackets(1))
        ));
        assert!(matches!(
            Interpreter::new("+]+", []),
            Err(Error::MismatchedBrackets(1))
        ));
    }

    #[test]
    fn detects_both_tape_boundaries() {
        assert_eq!(execute("<", &[], 10), Err(Error::BoundsError(0)));
        let source = ">".repeat(TAPE_SIZE);
        assert_eq!(
            execute(&source, &[], DEFAULT_STEP_LIMIT),
            Err(Error::BoundsError(TAPE_SIZE))
        );
    }

    #[test]
    fn can_pause_resume_and_enforce_a_budget() {
        let mut interpreter = Interpreter::new("++++.", []).unwrap();
        assert_eq!(interpreter.run(2).unwrap(), RunStatus::Running);
        assert_eq!(interpreter.tape()[0], 2);
        assert_eq!(interpreter.run(10).unwrap(), RunStatus::Finished);
        assert_eq!(interpreter.output(), [4]);
        assert_eq!(execute("+[]", &[], 100), Err(Error::StepLimitExceeded(100)));
    }

    #[test]
    fn ignores_comments_and_non_ascii_text() {
        assert_eq!(execute("hé++++.+", &[], 20).unwrap().output, [4]);
    }
}
