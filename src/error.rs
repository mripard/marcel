use std::io;
use std::result;

pub type Result<T> = result::Result<T, MarcelError>;

#[derive(Debug, Clone)]
pub enum MarcelError {
    IOError,
}

impl From<io::Error> for MarcelError {
    fn from(_error: io::Error) -> MarcelError {
        MarcelError::IOError
    }
}
