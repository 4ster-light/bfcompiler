use crate::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const MAX_PROG_SIZE: usize = 30_000;

pub fn read_bf_file(bf_path: &Path) -> anyhow::Result<String> {
    if !bf_path.exists() || !bf_path.is_file() {
        return Err(Error::InvalidPath(bf_path.display().to_string()).into());
    }

    if bf_path.extension().and_then(|ext| ext.to_str()) != Some("bf") {
        return Err(Error::InvalidExtension(bf_path.display().to_string()).into());
    }

    let mut bf_file =
        File::open(bf_path).map_err(|e| Error::FileRead(bf_path.display().to_string(), e))?;
    let mut bf_code = String::new();

    bf_file.read_to_string(&mut bf_code)?;
    Ok(bf_code)
}
