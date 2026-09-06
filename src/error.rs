use std::{fmt, io};

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::new(error.to_string())
    }
}

impl AppError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
