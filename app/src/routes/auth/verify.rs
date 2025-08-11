// app/src/routes/auth/verify.rs

use crate::errors::ApiError;
use crate::models::UserError;
use pavex::{get, Response, http::StatusCode};
use pavex::request::RequestHead;
use super::UserServiceContainer;

#[get(path = "/auth/verify")]
pub async fn verify_email(user_service: &UserServiceContainer, request_head: &RequestHead) -> Result<Response, ApiError> {
    // Extract token from query string
    let token = request_head
        .target
        .query()
        .and_then(|q| get_query_param(q, "token"))
        .ok_or_else(|| UserError::Validation { message: "Missing token".into() })?;
    let ok = user_service.0.verify_email(&token).await?;
    if ok {
        Ok(Response::new(StatusCode::FOUND).set_typed_body(String::new()))
    } else {
        Err(UserError::Validation { message: "Invalid or expired verification link".into() }.into())
    }
}

fn get_query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
            if k == key {
                return Some(v.to_string());
            }
        }
    }
    None
}


