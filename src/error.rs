use std::fmt;

#[derive(Debug)]
pub struct NotFoundError;

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can not found definitions of word")
    }
}

impl std::error::Error for NotFoundError {}

#[derive(Debug)]
pub struct ApiNotFoundError;

impl fmt::Display for ApiNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can not found definitions of word")
    }
}

impl std::error::Error for ApiNotFoundError {}
