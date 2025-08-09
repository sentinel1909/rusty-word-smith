// app/src/routes/auth/register.rs

// dependencies
use super::UserServiceContainer;
use crate::errors::ApiError;
use crate::models::{CreateUserRequest, UserResponse};
use pavex::{post, request::body::JsonBody};

// handler which will be called when the user visits the register page
#[post(path = "/auth/register")]
pub async fn register(
    body: &JsonBody<CreateUserRequest>,
    user_service: &UserServiceContainer,
) -> Result<UserResponse, ApiError> {
    let create_user_request = body.0.clone();

    let user_response = user_service.0.register(create_user_request).await?;

    Ok(user_response)
}
