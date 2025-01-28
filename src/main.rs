use bfcompiler::mode::Mode;
use bfcompiler::{
    compiler::build_bf,
    interpreter::interpret_bf,
    repl::repl_bf,
    utils::{parse_args, read_bf_file},
};
use std::io;

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
