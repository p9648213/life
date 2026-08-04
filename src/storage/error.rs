use std::fmt::{self};

#[derive(Debug)]
pub enum StoreError {
    ConnectionError(String),
    ReadError(String)
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
        }
    }
}
