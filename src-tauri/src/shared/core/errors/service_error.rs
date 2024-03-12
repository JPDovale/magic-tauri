use std::fmt;

#[derive(Debug)]
pub struct ServiceError {
    pub message: String,
    pub status: u16,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ServiceError {}

impl ServiceError {
    pub fn internal_error() -> Self {
        ServiceError {
            message: "Internal error".to_string(),
            status: 500,
        }
    }

    pub fn user_already_exists() -> Self {
        ServiceError {
            message: "User already exists".to_string(),
            status: 409,
        }
    }
}
