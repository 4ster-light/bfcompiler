use bfcompiler::error::Error;
use bfcompiler::utils::read_bf_file;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_invalid_extension() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("test.txt");

    fs::write(&path, "++++++.").unwrap();

    let result = read_bf_file(&path);

    assert!(matches!(
        result.unwrap_err().downcast_ref::<Error>(),
        Some(Error::InvalidExtension(_))
    ));
}

#[test]
fn test_valid_extension() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("test.bf");

    fs::write(&path, "++++++.").unwrap();

    let result = read_bf_file(&path);

    assert_eq!(result.unwrap(), "++++++.");
}
