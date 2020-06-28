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
