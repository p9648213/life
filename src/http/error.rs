use std::{
    fmt::{self},
    num::ParseIntError,
    str::Utf8Error,
};

#[derive(Debug)]
pub enum HttpError {
    InvalidUtf8(Utf8Error),
    MethodParseError,
    ContentLengthParseIntError(ParseIntError),
    ContentLengthSizeError,
    RequestHeaderInvalid,
    RequestLineInvalid,
    RequestHttpVersionInvalid,
    RequestRouteInvalid,
}

impl From<Utf8Error> for HttpError {
    fn from(err: Utf8Error) -> Self {
        HttpError::InvalidUtf8(err)
    }
}

impl From<ParseIntError> for HttpError {
    fn from(err: ParseIntError) -> Self {
        HttpError::ContentLengthParseIntError(err)
    }
}

impl std::error::Error for HttpError {}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::InvalidUtf8(err) => {
                write!(f, "Invalid UTF-8 in headers: {err}")
            }
            HttpError::MethodParseError => {
                write!(f, "Invalid Http Method")
            }
            HttpError::ContentLengthParseIntError(err) => {
                write!(f, "Invalid Content Length: {err}")
            }
            HttpError::ContentLengthSizeError => {
                write!(f, "Content Length Size Error")
            }
            HttpError::RequestHeaderInvalid => {
                write!(f, "Invalid Header")
            }
            HttpError::RequestLineInvalid => {
                write!(f, "Invalid Request Line")
            }
            HttpError::RequestHttpVersionInvalid => {
                write!(f, "Invalid Http Version")
            }
            HttpError::RequestRouteInvalid => {
                write!(f, "Invalid Route Url")
            }
        }
    }
}
