use std::io;
use std::result;
use std::num::{ParseIntError, ParseFloatError};

#[derive(Debug)]
pub enum Error {
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    IO(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
        Error::ParseFloatError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

