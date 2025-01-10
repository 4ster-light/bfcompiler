use std::io;

mod compiler;
mod interpreter;
mod mode;
mod repl;
mod util;

use compiler::build_bf;
use interpreter::interpret_bf;
use mode::Mode;
use repl::repl_bf;
use util::{parse_args, read_bf_file};

fn main() -> io::Result<()> {
    let (mode, bf_path, save_output) = parse_args();
    match mode {
        Mode::BUILD => build_bf(bf_path, save_output)?,
        Mode::RUN => interpret_bf(&read_bf_file(&bf_path)?)?,
        Mode::REPL => repl_bf()?,
        Mode::WRONG => {
            eprintln!("Invalid mode: {}", mode);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode"));
        }
    }
    Ok(())
}
