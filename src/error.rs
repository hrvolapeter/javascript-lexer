use std::{error, fmt, num};

/// Error that is returned if lexer fails
#[derive(Debug)]
pub enum Error {
    /// Lexer failed to process all input
    LexingIncomplete,
    /// Lexer failed for unknow reasons
    InternalError(Box<error::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {}

impl From<num::ParseIntError> for Error {
    fn from(o: num::ParseIntError) -> Error {
        Error::InternalError(box o)
    }
}
