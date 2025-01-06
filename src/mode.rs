use std::fmt::{Debug, Display, Formatter, Result};

#[derive(PartialEq)]
pub enum Mode {
    RUN,
    BUILD,
    REPL,
    WRONG,
}

impl Mode {
    pub fn from_str(str: &str) -> Self {
        match str {
            "run" => Mode::RUN,
            "build" => Mode::BUILD,
            "repl" => Mode::REPL,
            _ => Mode::WRONG,
        }
    }
}

impl Debug for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", format!("{}", self))
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mode = match self {
            Mode::RUN => "run",
            Mode::BUILD => "build",
            Mode::REPL => "repl",
            Mode::WRONG => "wrong",
        };
        write!(f, "{}", mode)
    }
}

impl PartialEq<&str> for Mode {
    fn eq(&self, other: &&str) -> bool {
        format!("{}", self) == *other
    }
}

impl PartialEq<Mode> for &str {
    fn eq(&self, other: &Mode) -> bool {
        *self == format!("{}", other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_from_str() {
        assert_eq!(Mode::from_str("run"), Mode::RUN);
        assert_eq!(Mode::from_str("build"), Mode::BUILD);
        assert_eq!(Mode::from_str("repl"), Mode::REPL);
        assert_eq!(Mode::from_str("wrong"), Mode::WRONG);
    }

    #[test]
    fn test_mode_debug() {
        assert_eq!(format!("{:?}", Mode::RUN), "run");
        assert_eq!(format!("{:?}", Mode::BUILD), "build");
        assert_eq!(format!("{:?}", Mode::REPL), "repl");
        assert_eq!(format!("{:?}", Mode::WRONG), "wrong");
    }

    #[test]
    fn test_mode_display() {
        assert_eq!(format!("{}", Mode::RUN), "run");
        assert_eq!(format!("{}", Mode::BUILD), "build");
        assert_eq!(format!("{}", Mode::REPL), "repl");
        assert_eq!(format!("{}", Mode::WRONG), "wrong");
    }
}
