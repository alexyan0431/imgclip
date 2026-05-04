use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Clipboard(String),
    Io(std::io::Error),
    Args(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Clipboard(msg) => write!(f, "{msg}"),
            AppError::Io(err) => write!(f, "{err}"),
            AppError::Args(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<arboard::Error> for AppError {
    fn from(err: arboard::Error) -> Self {
        AppError::Clipboard(err.to_string())
    }
}

impl From<lexopt::Error> for AppError {
    fn from(err: lexopt::Error) -> Self {
        AppError::Args(err.to_string())
    }
}
