use std::fmt;

pub enum AuthError {
    JwtNotFound,
    JwtInvalid,
}

impl AuthError {
    pub fn new(message: &str) -> AuthError {
        match message {
            "JWT not found in the Authorization header" => AuthError::JwtNotFound,
            _ => AuthError::JwtInvalid,
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::JwtNotFound => write!(f, "JWT not found"),
            AuthError::JwtInvalid => write!(f, "Invalid JWT"),
        }
    }
}

impl fmt::Debug for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for AuthError {}
