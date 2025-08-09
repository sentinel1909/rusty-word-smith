// app/src/routes/auth/login.rs

// dependencies
use crate::authorization::{USERNAME, USER_ID, USER_ROLE};
use crate::errors::ApiError;
use crate::models::{LoginRequest, UserSummary};
use crate::response::ApiResponse;
use pavex::{post, request::body::JsonBody};
use pavex_session::Session;
use super::UserServiceContainer;

// handler which will be called when the user visits the login page
#[post(path = "/auth/login")]
pub async fn login(
    body: &JsonBody<LoginRequest>,
    session: &mut Session<'_>,
    user_service: &UserServiceContainer,
) -> Result<ApiResponse<UserSummary>, ApiError> {
    let login_request = body.0.clone();
    let user_summary = user_service.0.login(login_request).await?;

    session.cycle_id();

    session.insert(USER_ID, user_summary.id).await.unwrap();
    session
        .insert(USERNAME, user_summary.username.clone())
        .await
        .unwrap();
    session
        .insert(USER_ROLE, user_summary.role)
        .await
        .unwrap();

    Ok(ApiResponse::ok(user_summary))
}
