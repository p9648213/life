use std::{fmt, num::ParseIntError, str::Utf8Error};

#[derive(Debug)]
pub enum AppError {
    InvalidUtf8(Utf8Error),
    MethodParseError,
    ContentLengthParseIntError(ParseIntError),
    ContentLengthSizeError,
    RequestHeaderInvalid,
    RequestLineInvalid,
    RequestHttpVersionInvalid,
}

impl From<Utf8Error> for AppError {
    fn from(err: Utf8Error) -> Self {
        AppError::InvalidUtf8(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::ContentLengthParseIntError(err)
    }
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidUtf8(err) => {
                write!(f, "Invalid UTF-8 in headers: {err}")
            }
            AppError::MethodParseError => {
                write!(f, "Invalid Http Method")
            }
            AppError::ContentLengthParseIntError(err) => {
                write!(f, "Invalid Content Length: {err}")
            }
            AppError::ContentLengthSizeError => {
                write!(f, "Content Length Size Error")
            }
            AppError::RequestHeaderInvalid => {
                write!(f, "Invalid Header")
            }
            AppError::RequestLineInvalid => {
                write!(f, "Invalid Request Line")
            }
            AppError::RequestHttpVersionInvalid => {
                write!(f, "Invalid Http Version")
            }
        }
    }
}
