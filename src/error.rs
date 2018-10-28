use std::io;
use std::result;

use serde_yaml as yaml;

pub type Result<T> = result::Result<T, MarcelError>;

#[derive(Debug, Clone)]
pub enum MarcelError {
    IOError,
    TraceError,
    YAMLError,
}

impl From<io::Error> for MarcelError {
    fn from(_error: io::Error) -> MarcelError {
        MarcelError::IOError
    }
}

impl From<yaml::Error> for MarcelError {
    fn from(_error: yaml::Error) -> MarcelError {
        MarcelError::YAMLError
    }
}
