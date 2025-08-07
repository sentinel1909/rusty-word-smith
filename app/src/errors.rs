// app/src/errors.rs


// dependencies
use crate::response::{ApiResponse, Status};
use crate::models::UserError;
use pavex::{
    error_handler,
    http::{StatusCode},    
    Response,
    time::Timestamp,
};
use serde::Serialize;
use std::convert::From;
use thiserror::Error;

// enum type to represent a public facing error
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Template error: {0}")]
    TemplateError(#[from] pavex_tera_template::TemplateError),

    #[error("Static file error: {0}")]
    StaticFileError(#[from] pavex_static_files::ServeError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("User error: {0}")]
    UserError(#[from] UserError),
}

// The error‑side of an API response never carries data, so we just use
// `()` as the type parameter.
impl From<&ApiError> for ApiResponse<()> {
    fn from(err: &ApiError) -> Self {
        // Determine the HTTP status code and the “status tag” we want to send in
        // the envelope.
        let (status_code, status_tag) = match err {
            ApiError::TemplateError(_) => (StatusCode::INTERNAL_SERVER_ERROR, Status::Error),
            ApiError::StaticFileError(e) => {
                let lower = e.to_string().to_lowercase();
                if lower.contains("not found") || lower.contains("no such file") {
                    (StatusCode::NOT_FOUND, Status::Error)
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, Status::Error)
                }
            }
            ApiError::SerializationError(_) => (StatusCode::INTERNAL_SERVER_ERROR, Status::Error),
            ApiError::UserError(user_err) => match user_err {
                UserError::Validation { .. } => (StatusCode::BAD_REQUEST, Status::Error),
                UserError::UserNotFound => (StatusCode::NOT_FOUND, Status::Error),
                UserError::EmailExists | UserError::UsernameExists => {
                    (StatusCode::CONFLICT, Status::Error)
                }
                UserError::InvalidCredentials => (StatusCode::UNAUTHORIZED, Status::Error),
                // Any other variant is treated as an internal server error.
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Status::Error),
            },
        };

        ApiResponse {
            status: status_tag,
            code: Some(status_code.as_u16()),
            message: Some(err.to_string()),
            data: None,
            timestamp: Timestamp::now(),
        }
    }
}

// helper that turns an ApiResponse into a Pavex Response
impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Consume the envelope and produce a `pavex::Response`
    /// (sets the correct status code and JSON body).
    pub fn into_response(self) -> Response {
        // Prefer the numeric `code` field; fall back to the enum variant.
        let status = self
            .code
            .and_then(|c| StatusCode::from_u16(c).ok())
            .unwrap_or(match self.status {
                Status::Ok => StatusCode::OK,
                Status::Error => StatusCode::INTERNAL_SERVER_ERROR,
            });

        let json = serde_json::to_string(&self).unwrap_or_else(|_| {
            r#"{"msg":"Error","status":500,"details":"Internal server error formatting error response"}"#.to_string()
        });

        Response::new(status).set_typed_body(json)
    }
}

// This function is called automatically by Pavex whenever an `ApiError`
// propagates to the top level of the request handling chain.
#[error_handler]
pub fn api_error2response(error: &ApiError) -> Response {
    // 1️⃣ Turn the error into the envelope
    let envelope = ApiResponse::<()>::from(error);

    // 2️⃣ Create the real HTTP response
    envelope.into_response()
}

