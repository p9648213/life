use core::fmt;

#[derive(Debug)]
pub enum TemplateError {
    EmptyVariable,
    UnCloseVariable,
    MissingOpenVariable,
    InvalidVariable,
    InvalidOperation,
}

impl std::error::Error for TemplateError {}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::EmptyVariable => {
                write!(f, "Empty variable between {{}}")
            }
            TemplateError::UnCloseVariable => {
                write!(f, "Missing variable closing }}")
            }
            TemplateError::MissingOpenVariable => {
                write!(f, "Missing variable open {{")
            }
            TemplateError::InvalidVariable => {
                write!(f, "Invalid Variable")
            }
            TemplateError::InvalidOperation => {
                write!(f, "Invalid Operation")
            }
        }
    }
}
