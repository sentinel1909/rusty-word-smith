// app/src/response.rs

// dependencies
use pavex::time::Timestamp;
use serde::{Deserialize, Serialize};

// enum type to represent the status
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Error,
}

// struct type to represent an API response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub timestamp: Timestamp,
}

// constructors (static factories)
impl<T> ApiResponse<T> {
    // private helper that builds the constant type of every response.
    fn base(status: Status, message: Option<String>, code: Option<u16>) -> Self {
        Self {
            status,
            code,
            message,
            data: None,
            timestamp: Timestamp::now(),
        }
    }

    // success response "with"a payload
    #[must_use]
    pub fn ok(data: T) -> Self {
        Self {
            data: Some(data),
            ..Self::base(Status::Ok, None, None)
        }
    }

    // Success response *with* a payload *and* a human‑readable message.
    #[must_use]
    pub fn ok_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            data: Some(data),
            ..Self::base(Status::Ok, Some(message.into()), None)
        }
    }

    // Success response *with* a payload and an optional numeric code.
    #[must_use]
    pub fn ok_with_code(data: T, code: u16) -> Self {
        Self {
            data: Some(data),
            ..Self::base(Status::Ok, None, Some(code))
        }
    }

    // Success response *with* payload, message and code – the most general variant.
    #[must_use]
    pub fn ok_full(
        data: T,
        message: impl Into<String>,
        code: u16,
    ) -> Self {
        Self {
            data: Some(data),
            ..Self::base(Status::Ok, Some(message.into()), Some(code))
        }
    }

    // Minimal error response: just a message.
    #[must_use]
    pub fn err(message: impl Into<String>) -> Self {
        Self::base(Status::Error, Some(message.into()), None)
    }

    // Error response with a numeric code (e.g. HTTP status code or application‑specific error code).
    #[must_use]
    pub fn err_with_code(message: impl Into<String>, code: u16) -> Self {
        Self::base(Status::Error, Some(message.into()), Some(code))
    }
}

// A trait that your error type must implement to be turned into an `ApiResponse`.
pub trait IntoApiError {
    fn code(&self) -> Option<u16>;
    fn message(&self) -> String;
}

impl<T, E> From<Result<T, E>> for ApiResponse<T>
where
    E: IntoApiError,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(payload) => ApiResponse::ok(payload),
            Err(err) => ApiResponse::err_with_code(err.message(), err.code().unwrap_or(0)),
        }
    }
}