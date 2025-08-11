// app/src/routes/auth/resend.rs

// dependencies
use crate::errors::ApiError;
use super::UserServiceContainer;
use pavex::{post, request::body::JsonBody, Response};
use serde::Deserialize;

// struct type to represent a resent request
#[derive(Deserialize)]
pub struct ResendRequest {
    pub email: String,
}

#[post(path = "/auth/resend-verification")]
pub async fn resend_verification(
    body: &JsonBody<ResendRequest>,
    user_service: &UserServiceContainer,
) -> Result<Response, ApiError> {
    let email = body.0.email.clone();
    // Delegate to service, which enforces rate-limiting and token issuance
    match user_service.0.resend_verification(&email).await {
        Ok(()) => Ok(Response::no_content()),
        Err(e) => {
            // If rate-limited, return 429; else map to generic OK to avoid enumeration
            if matches!(e, crate::models::UserError::Validation { .. }) {
                Ok(Response::too_many_requests())
            } else {
                Ok(Response::no_content())
            }
        }
    }
}


