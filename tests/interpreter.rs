use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_interpreter_prints_ascii_6() {
    let code = "++++++.";
    let output = capture_interpreter_output(code);

    assert_eq!(output, vec![6, b'\n']);
}

#[test]
fn test_interpreter_hello_world() {
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let output = capture_interpreter_output(code);

    assert_eq!(output, b"Hello World!\n\n");
}

fn capture_interpreter_output(code: &str) -> Vec<u8> {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("test.bf");

    fs::write(&path, code).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "run", path.to_str().unwrap()])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .expect("Failed to execute cargo run");

    if !output.status.success() {
        panic!(
            "cargo run failed: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    output
        .stdout
        .into_iter()
        .take_while(|&b| b != b'E') // Strip "Execution successful!"
        .collect()
}
