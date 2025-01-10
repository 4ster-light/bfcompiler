use crate::mode::Mode;
use std::env::args;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::path::PathBuf;

pub const MAX_PROG_SIZE: usize = 30_000;

pub fn read_bf_file(bf_path: &Path) -> io::Result<String> {
    let mut bf_file = File::open(bf_path)?;
    let mut bf_code = String::new();
    bf_file.read_to_string(&mut bf_code)?;
    Ok(bf_code)
}

pub fn parse_args() -> (Mode, PathBuf, bool) {
    let args: Vec<String> = args().collect();
    let mode = args.get(1).map_or(Mode::WRONG, |m| Mode::from_str(m));
    let bf_path = args
        .get(2)
        .map_or(PathBuf::new(), |p| Path::new(p).to_path_buf());
    let save_output = args.len() > 3 && (args[3] == "-s" || args[3] == "--save");

    (mode, bf_path, save_output)
}
