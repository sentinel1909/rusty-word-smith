// app/src/routes/auth/whoami.rs

// dependencies
use crate::authorization::{USERNAME, USER_ID, USER_ROLE};
use crate::errors::ApiError;
use crate::models::{UserRole, WhoAmIResponse};
use crate::response::ApiResponse;
use pavex::get;
use pavex_session::Session;
use uuid::Uuid;

#[get(path = "/auth/whoami")]
pub async fn whoami(session: &Session<'_>) -> Result<ApiResponse<WhoAmIResponse>, ApiError> {
    let user_id: Option<Uuid> = session.get(USER_ID).await.unwrap();
    let username: Option<String> = session.get(USERNAME).await.unwrap();
    let role: Option<UserRole> = session.get(USER_ROLE).await.unwrap();

    match (user_id, username, role) {
        (Some(id), Some(username), Some(role)) => Ok(ApiResponse::ok(WhoAmIResponse { id, username, role })),
        _ => Err(ApiError::Unauthorized("Invalid session".to_string())),
    }
}
