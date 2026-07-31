use std::fmt;

#[derive(Debug)]
pub enum StoreError {
    ConnectionError(String),
}

impl std::error::Error for StoreError {}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::ConnectionError(err) => {
                write!(f, "Error connecting storage: {err}")
            }
        }
    }
}
