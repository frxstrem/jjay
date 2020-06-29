use std::fmt::{self, Display};

use crate::ast::Rule;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone, Debug)]
pub enum ParseError {
    Pest(pest::error::Error<Rule>),
    Other(String),
}

impl Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Pest(err) => write!(fmt, "{}", err),
            ParseError::Other(msg) => write!(fmt, "{}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(err: pest::error::Error<Rule>) -> ParseError {
        ParseError::Pest(err)
    }
}

pub type ScriptResult<T> = Result<T, ScriptError>;

#[derive(Clone, Debug)]
pub enum ScriptError {
    VariableNotFound(String),
    VariableAlreadyExists(String),
    NotStringConvertible,
    NotCallable,

    Parse(ParseError),
    Other(String),
}

impl Display for ScriptError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScriptError::VariableNotFound(name) => write!(fmt, "Variable not found: {}", name),
            ScriptError::VariableAlreadyExists(name) => {
                write!(fmt, "Variable already exists: {}", name)
            }
            ScriptError::NotStringConvertible => write!(fmt, "Cannot convert value to string"),
            ScriptError::NotCallable => write!(fmt, "Cannot call value"),

            ScriptError::Parse(err) => write!(fmt, "{}", err),
            ScriptError::Other(msg) => write!(fmt, "{}", msg),
        }
    }
}

impl std::error::Error for ScriptError {}

impl From<ParseError> for ScriptError {
    fn from(err: ParseError) -> ScriptError {
        ScriptError::Parse(err)
    }
}

#[allow(unused_macros)]
macro_rules! script_error {
    ($($args:tt)*) => {
        ScriptError::Other(format!($($args)*))
    }
}
