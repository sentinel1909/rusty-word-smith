// app/src/models/user/dto.rs

// dependencies
use super::{User, UserRole};
use crate::errors::ApiError;
use pavex::{IntoResponse, Response, time::Timestamp};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use uuid::Uuid;
use validator::Validate;

/// Request DTO for user registration
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    #[validate(regex(
        path = "USERNAME_REGEX",
        message = "Username can only contain letters, numbers, and underscores"
    ))]
    pub username: String,

    #[validate(email(message = "Invalid email address"))]
    #[validate(length(max = 255, message = "Email cannot exceed 255 characters"))]
    pub email: String,

    #[validate(length(min = 8, max = 128, message = "Password must be 8-128 characters"))]
    pub password: String,

    #[validate(length(max = 100, message = "Display name cannot exceed 100 characters"))]
    pub display_name: Option<String>,
}

/// Request DTO for user updates
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(max = 100, message = "Display name cannot exceed 100 characters"))]
    pub display_name: Option<String>,

    #[validate(length(max = 1000, message = "Bio cannot exceed 1000 characters"))]
    pub bio: Option<String>,

    #[validate(url(message = "Avatar URL must be valid"))]
    #[validate(length(max = 500, message = "Avatar URL cannot exceed 500 characters"))]
    pub avatar_url: Option<String>,

    #[validate(length(max = 100, message = "Twitter handle cannot exceed 100 characters"))]
    pub social_twitter: Option<String>,

    #[validate(length(max = 100, message = "GitHub username cannot exceed 100 characters"))]
    pub social_github: Option<String>,

    #[validate(url(message = "Website URL must be valid"))]
    #[validate(length(max = 500, message = "Website URL cannot exceed 500 characters"))]
    pub website_url: Option<String>,
}

/// Public user response DTO - safe for API responses
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String, // In a real app, you might want to mask this
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub is_active: bool,
    pub email_verified: bool,
    pub social_twitter: Option<String>,
    pub social_github: Option<String>,
    pub website_url: Option<String>,
    pub created_at: Timestamp,
}

/// Summary user response for listings
#[derive(Debug, Serialize)]
pub struct UserSummary {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
}

/// Login request DTO
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username or email is required"))]
    pub username_or_email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

/// Password change request DTO
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,

    #[validate(length(min = 8, max = 128, message = "New password must be 8-128 characters"))]
    pub new_password: String,
}

// Validation regex - you'd put this in a constants module
static USERNAME_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());

// Conversion implementations
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            bio: user.bio,
            avatar_url: user.avatar_url,
            role: user.role,
            is_active: user.is_active,
            email_verified: user.email_verified,
            social_twitter: user.social_twitter,
            social_github: user.social_github,
            website_url: user.website_url,
            created_at: user.created_at,
        }
    }
}

impl From<User> for UserSummary {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            role: user.role,
        }
    }
}

// implement the IntoResponse trait for UserResponse
impl IntoResponse for UserResponse {
    fn into_response(self) -> Response {
        match serde_json::to_string(&self) {
            Ok(body) => Response::ok().set_typed_body(body),
            Err(err) => crate::errors::api_error2response(&ApiError::SerializationError(err)),
        }
    }
}
