use std::fmt::{self, Display};

use crate::ast::Rule;
use crate::value::ValueType;

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

#[derive(Debug)]
pub enum ScriptError {
    VariableNotFound(String),
    VariableAlreadyExists(String),
    NotStringConvertible(ValueType),
    NotIntConvertible(ValueType),
    NotCallable(ValueType),
    PropertyNotFound(ValueType, String),

    Parse(ParseError),
    Io(std::io::Error),
    Other(String),
}

impl Display for ScriptError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScriptError::VariableNotFound(name) => write!(fmt, "Variable not found: {}", name),
            ScriptError::VariableAlreadyExists(name) => {
                write!(fmt, "Variable already exists: {}", name)
            }
            ScriptError::NotStringConvertible(value_type) => {
                write!(fmt, "Cannot convert {} to string", value_type)
            }
            ScriptError::NotIntConvertible(value_type) => {
                write!(fmt, "Cannot convert {} to integer", value_type)
            }
            ScriptError::NotCallable(value_type) => write!(fmt, "Cannot call {}", value_type),
            ScriptError::PropertyNotFound(value_type, key) => {
                write!(fmt, "Value {} has no property {:?}", value_type, key)
            }

            ScriptError::Parse(err) => write!(fmt, "{}", err),
            ScriptError::Io(err) => write!(fmt, "{}", err),
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

impl From<std::io::Error> for ScriptError {
    fn from(err: std::io::Error) -> ScriptError {
        ScriptError::Io(err)
    }
}

#[allow(unused_macros)]
macro_rules! script_error {
    ($($args:tt)*) => {
        ScriptError::Other(format!($($args)*))
    }
}
