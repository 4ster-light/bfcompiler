use bfcompiler::mode::Mode;

#[test]
fn test_mode_display() {
    assert_eq!(format!("{}", Mode::RUN), "run");
    assert_eq!(format!("{}", Mode::BUILD), "build");
    assert_eq!(format!("{}", Mode::REPL), "repl");
    assert_eq!(format!("{}", Mode::WRONG), "wrong");
}
