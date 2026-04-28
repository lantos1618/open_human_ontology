use std::fmt;

pub mod genetics;

#[derive(Debug, Clone, PartialEq)]
pub enum BiologyError {
    InvalidValue(String),
    InvalidState(String),
    InvalidParameter(String),
}

impl fmt::Display for BiologyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BiologyError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            BiologyError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            BiologyError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
        }
    }
}

impl std::error::Error for BiologyError {}

pub type BiologyResult<T> = Result<T, BiologyError>;
