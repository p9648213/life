use std::{array::TryFromSliceError, fmt::{self}, str::Utf8Error};

#[derive(Debug)]
pub enum StoreError {
    ConnectionError(String),
    ReadError(String),
    FieldTooLarge(String),
    InvalidUtf8(Utf8Error),
    TryFromSliceError(TryFromSliceError)
}

impl From<Utf8Error> for StoreError {
    fn from(err: Utf8Error) -> Self {
        StoreError::InvalidUtf8(err)
    }
}

impl From<TryFromSliceError> for StoreError {
    fn from(err: TryFromSliceError) -> Self {
        StoreError::TryFromSliceError(err)
    }
}

impl std::error::Error for StoreError {}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::ConnectionError(err) => {
                write!(f, "Error connecting storage: {err}")
            }
            StoreError::ReadError(err) => {
                write!(f, "Read error: {err}")
            }
            StoreError::FieldTooLarge(err) => {
                write!(f, "Field Too Large: {err}")
            }
            StoreError::InvalidUtf8(err) => {
                write!(f, "Invalid UTF-8 In Payload: {err}")
            }
            StoreError::TryFromSliceError(err) => {
                write!(f, "try_from error: {err}")
            }
        }
    }
}
