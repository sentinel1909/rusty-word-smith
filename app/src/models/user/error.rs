// app/src/models/user/error.rs

use crate::response::IntoApiError;
use pavex::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("User not found")]
    UserNotFound,

    #[error("Email already exists")]
    EmailExists,

    #[error("Username already exists")]
    UsernameExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Email not verified")]
    EmailNotVerified,

    #[error("Password hashing error: {0}")]
    PasswordHash(String),
}

impl IntoApiError for UserError {
    fn code(&self) -> Option<u16> {
        match self {
            UserError::Validation { .. } => Some(StatusCode::BAD_REQUEST.as_u16()),
            UserError::UserNotFound => Some(StatusCode::NOT_FOUND.as_u16()),
            UserError::EmailExists | UserError::UsernameExists => {
                Some(StatusCode::CONFLICT.as_u16())
            }
            UserError::InvalidCredentials | UserError::EmailNotVerified => {
                Some(StatusCode::UNAUTHORIZED.as_u16())
            }
            UserError::Database(_) | UserError::PasswordHash(_) => {
                Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16())
            }
        }
    }

    fn message(&self) -> String {
        self.to_string()
    }
}
