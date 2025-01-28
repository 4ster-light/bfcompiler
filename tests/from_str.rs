use bfcompiler::mode::Mode;

#[test]
fn test_mode_from_str() {
    assert_eq!(Mode::from_str("run"), Mode::RUN);
    assert_eq!(Mode::from_str("build"), Mode::BUILD);
    assert_eq!(Mode::from_str("repl"), Mode::REPL);
    assert_eq!(Mode::from_str("wrong"), Mode::WRONG);
}
