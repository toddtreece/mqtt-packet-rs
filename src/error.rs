use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::num::{ParseIntError, TryFromIntError};
use std::string::FromUtf8Error;

/// Error type used in all `Result<T, E>` return values.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
  ParseError,
  GenerateError,
}

impl StdError for Error {
  fn description(&self) -> &str {
    match *self {
      Error::ParseError => "Unable to parse type",
      Error::GenerateError => "Unable to generate data",
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::ParseError => f.write_str("ParseError"),
      Error::GenerateError => f.write_str("GenerateError"),
    }
  }
}

impl From<IoError> for Error {
  fn from(e: IoError) -> Self {
    match e {
      _ => Error::ParseError,
    }
  }
}

impl From<FromUtf8Error> for Error {
  fn from(e: FromUtf8Error) -> Self {
    match e {
      _ => Error::ParseError,
    }
  }
}

impl From<TryFromIntError> for Error {
  fn from(e: TryFromIntError) -> Self {
    match e {
      _ => Error::ParseError,
    }
  }
}

impl From<ParseIntError> for Error {
  fn from(e: ParseIntError) -> Self {
    match e {
      _ => Error::ParseError,
    }
  }
}
