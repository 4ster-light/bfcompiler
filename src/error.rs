use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read Brainfuck file '{0}': {1}")]
    FileRead(String, #[source] std::io::Error),

    #[error("Memory access out of bounds at pointer {0}")]
    BoundsError(usize),

    #[error("Mismatched brackets in code at position {0}")]
    MismatchedBrackets(usize),

    #[error("Compilation to Rust code failed: {0}")]
    CompilationFailed(String),

    #[error("Invalid Brainfuck file path: {0}")]
    InvalidPath(String),

    #[error("File '{0}' does not have a .bf extension")]
    InvalidExtension(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
