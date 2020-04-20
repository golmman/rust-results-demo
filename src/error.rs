// same idea as std::io::Error

use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Error {
            kind,
            message: String::from(message),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(ErrorKind::from(error.kind()), "test")
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    IoErrorKind(std::io::ErrorKind),
    Negative,
    Positive,
    BaseNameTooLong,
    ExtensionTooLong,
}

impl From<std::io::ErrorKind> for ErrorKind {
    fn from(error_kind: std::io::ErrorKind) -> Self {
        ErrorKind::IoErrorKind(error_kind)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
