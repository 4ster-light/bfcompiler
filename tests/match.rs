use bf_interpreter::{execute, Error, DEFAULT_STEP_LIMIT};

#[test]
fn test_mismatched_brackets() {
    let result = execute("+++[+++", &[], DEFAULT_STEP_LIMIT);
    assert!(matches!(result, Err(Error::MismatchedBrackets(_))));
}

#[test]
fn test_bounds_error() {
    let result = execute("<", &[], DEFAULT_STEP_LIMIT);
    assert!(matches!(result, Err(Error::BoundsError(_))));
}
