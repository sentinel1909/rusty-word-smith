// app/src/routes/auth/logout.rs

// dependencies
use crate::{errors::ApiError, response::ApiResponse};
use pavex::post;
use pavex_session::Session;

// handler which will be called when the user visits the login page
#[post(path = "/auth/logout")]
pub async fn logout(
    session: &mut Session<'_>,
) -> Result<ApiResponse<()>, ApiError> {
    session.clear().await.unwrap();
    session.cycle_id();

    Ok(ApiResponse::ok_with_message((), "Logged out successfully"))
}