use std::fmt;

#[derive(Debug)]
pub struct RepositoryError {
    message: String,
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RepositoryError {}
