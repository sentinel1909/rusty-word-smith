// app/src/routes/auth/verify.rs

// dependencies
use crate::errors::ApiError;
use crate::models::UserError;
use pavex::{get, Response, http::StatusCode};
use pavex::request::query::QueryParams;
use super::UserServiceContainer;

#[derive(serde::Deserialize)]
pub struct VerifyParams {
    pub token: String,
}

#[get(path = "/auth/verify")]
pub async fn verify_email(
    user_service: &UserServiceContainer,
    params: &QueryParams<VerifyParams>,
) -> Result<Response, ApiError> {
    let token = &params.0.token;
    let ok = user_service.0.verify_email(token).await?;
    if ok {
        Ok(Response::new(StatusCode::FOUND).set_typed_body(String::new()))
    } else {
        Err(UserError::Validation { message: "Invalid or expired verification link".into() }.into())
    }
}


