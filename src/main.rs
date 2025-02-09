use bfcompiler::mode::Mode;
use bfcompiler::{
    compiler::build_bf,
    interpreter::interpret_bf,
    repl::repl_bf,
    utils::{parse_args, read_bf_file},
};
use colored::*;
use std::io;

fn main() -> io::Result<()> {
    let (mode, bf_path, save_output) = parse_args();
    match mode {
        Mode::BUILD => match build_bf(bf_path, save_output) {
            Ok(_) => println!("{}", "Build successful!".green().bold()),
            Err(e) => eprintln!("{}: {}", "Build failed".red(), e),
        },
        Mode::RUN => match interpret_bf(&read_bf_file(&bf_path)?) {
            Ok(_) => println!("{}", "Execution successful!".green().bold()),
            Err(e) => eprintln!("{}: {}", "Execution failed".red(), e),
        },
        Mode::REPL => {
            if let Err(e) = repl_bf() {
                eprintln!("{}: {}", "REPL failed".red(), e);
            }
        }
        Mode::WRONG => {
            eprintln!("{}", "No compiler mode matches the given mode".red());
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode"));
        }
    }
    Ok(())
}
