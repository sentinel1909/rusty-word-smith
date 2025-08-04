// app/src/models/user/entity.rs

// dependencies
use pavex::time::Timestamp;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

/// User roles enum that matches your database enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Editor,
    Author,
    Contributor,
    Subscriber,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Subscriber
    }
}

impl UserRole {
    /// Parse role from string - useful for manual row mapping
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "admin" => Some(Self::Admin),
            "editor" => Some(Self::Editor),
            "author" => Some(Self::Author),
            "contributor" => Some(Self::Contributor),
            "subscriber" => Some(Self::Subscriber),
            _ => None,
        }
    }
}

/// Core User entity that maps directly to your database table
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,

    /// Password hash - never serialize this field
    #[serde(skip_serializing)]
    pub password_hash: String,

    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub is_active: bool,
    pub email_verified: bool,

    // Email verification fields
    pub email_verification_token: Option<String>,
    pub email_verification_expires_at: Option<Timestamp>,

    // Password reset fields
    pub password_reset_token: Option<String>,
    pub password_reset_expires_at: Option<Timestamp>,

    // Social media fields - keeping basic ones for MVB
    pub social_twitter: Option<String>,
    pub social_github: Option<String>,
    pub website_url: Option<String>,

    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl User {
    /// Check if user can perform admin actions
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }

    /// Check if user can create/edit content
    pub fn can_write(&self) -> bool {
        matches!(
            self.role,
            UserRole::Admin | UserRole::Editor | UserRole::Author
        )
    }

    /// Check if user can moderate comments
    pub fn can_moderate(&self) -> bool {
        matches!(self.role, UserRole::Admin | UserRole::Editor)
    }

    /// Get display name, falling back to username
    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.username)
    }
}
