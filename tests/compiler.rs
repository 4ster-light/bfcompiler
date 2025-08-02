use bfcompiler::compiler::build_bf;
use std::fs;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_compiler_prints_ascii_6() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("test.bf");
    let output_bin = temp_dir.path().join("test");
    let output_rs = temp_dir.path().join("test.rs");

    fs::write(&path, "++++++.").unwrap();

    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        "++++++.",
        "Input file contents mismatch"
    );

    let _ = fs::remove_file(&output_bin);
    let _ = fs::remove_file(&output_rs);

    if let Err(e) = build_bf(path.to_path_buf(), true) {
        let output_rs_content = fs::read_to_string(&output_rs).unwrap_or_default();
        panic!(
            "build_bf failed for ascii_6: {}\nGenerated output.rs:\n{}",
            e, output_rs_content
        );
    }

    sleep(Duration::from_millis(200)); // Wait for file system

    let output = Command::new(&output_bin)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .unwrap_or_else(|e| panic!("Failed to run {}: {}", output_bin.display(), e));

    assert_eq!(
        output.stdout,
        vec![6],
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_compiler_hello_world() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("hello.bf");
    let output_bin = temp_dir.path().join("hello");
    let output_rs = temp_dir.path().join("hello.rs");

    fs::write(&path, "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.").unwrap();

    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
        "Input file contents mismatch"
    );

    let _ = fs::remove_file(&output_bin);
    let _ = fs::remove_file(&output_rs);

    if let Err(e) = build_bf(path.to_path_buf(), true) {
        let output_rs_content = fs::read_to_string(&output_rs).unwrap_or_default();
        panic!(
            "build_bf failed for hello_world: {}\nGenerated output.rs:\n{}",
            e, output_rs_content
        );
    }

    sleep(Duration::from_millis(200)); // Wait for file system

    let output = Command::new(&output_bin)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .unwrap_or_else(|e| panic!("Failed to run {}: {}", output_bin.display(), e));

    assert_eq!(
        output.stdout,
        b"Hello World!\n",
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
