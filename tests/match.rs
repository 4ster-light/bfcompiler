use bfcompiler::error::Error;
use bfcompiler::interpreter::interpret_bf;

#[test]
fn test_mismatched_brackets() {
    let code = "+++[+++";
    let result = interpret_bf(code);

    assert!(matches!(
        result.unwrap_err().downcast_ref::<Error>(),
        Some(Error::MismatchedBrackets(_))
    ));
}

#[test]
fn test_bounds_error() {
    let code = "<";
    let result = interpret_bf(code);

    assert!(matches!(
        result.unwrap_err().downcast_ref::<Error>(),
        Some(Error::BoundsError(_))
    ));
}
