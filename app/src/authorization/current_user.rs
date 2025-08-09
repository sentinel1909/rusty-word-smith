// app/src/authorization/current_user.rs

// dependencies
use crate::authorization::{USER_ID, USERNAME, USER_ROLE};
use pavex::methods;
use pavex_session::Session;
use crate::{errors::ApiError, models::UserRole};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CurrentUser {
  pub id: Uuid,
  pub username: String,
  pub role: UserRole,
}

#[methods]
impl CurrentUser {
  #[request_scoped]
  pub async fn new(session: &Session<'_>) -> Result<Self, ApiError> {
    let id: Option<Uuid> = session.get(USER_ID).await.unwrap_or(None);
    let username: Option<String> = session.get(USERNAME).await.unwrap_or(None);
    let role: Option<UserRole> = session.get(USER_ROLE).await.unwrap_or(None);

    match (id, username, role) {
      (Some(id), Some(username), Some(role)) => Ok(Self { id, username, role }),
      _ => Err(ApiError::Unauthorized("Invalid session".into())),
    }
  }
}

