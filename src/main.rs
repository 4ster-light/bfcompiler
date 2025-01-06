use std::env::args;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

mod compiler;
mod interpreter;
mod mode;
mod repl;

use compiler::compile_bf;
use interpreter::interpret_bf;
use mode::Mode;
use repl::repl_bf;

pub const MAX_PROG_SIZE: usize = 30_000;

fn read_bf_file(bf_path: &Path) -> io::Result<String> {
    let mut bf_file = File::open(bf_path)?;
    let mut bf_code = String::new();
    bf_file.read_to_string(&mut bf_code)?;
    Ok(bf_code)
}

fn parse_args() -> (Mode, PathBuf, bool) {
    let args: Vec<String> = args().collect();
    let mode = args.get(1).map_or(Mode::WRONG, |m| Mode::from_str(m));
    let bf_path = args
        .get(2)
        .map_or(PathBuf::new(), |p| Path::new(p).to_path_buf());
    let save_output = args.len() > 3 && (args[3] == "-s" || args[3] == "--save");

    (mode, bf_path, save_output)
}

fn main() -> io::Result<()> {
    let (mode, bf_path, save_output) = parse_args();

    match mode {
        Mode::BUILD => {
            match File::create("output.rs")?
                .write_all(compile_bf(&read_bf_file(&bf_path)?).as_bytes())
            {
                Ok(_) => {
                    let status = Command::new("rustc")
                        .arg("output.rs")
                        .arg("-o")
                        .arg("output")
                        .status()?;
                    if status.success() {
                        if !save_output {
                            std::fs::remove_file("output.rs")?;
                        }
                    } else {
                        return Err(io::Error::new(io::ErrorKind::Other, "Compilation failed"));
                    }
                }
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::ReadOnlyFilesystem,
                        "Could not write to file",
                    ));
                }
            }
        }
        Mode::RUN => interpret_bf(&read_bf_file(&bf_path)?)?,
        Mode::REPL => repl_bf()?,
        Mode::WRONG => {
            eprintln!("Invalid mode: {}", mode);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode"));
        }
    }

    Ok(())
}
