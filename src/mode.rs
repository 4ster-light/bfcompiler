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
