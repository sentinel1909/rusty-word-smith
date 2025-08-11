// app/src/models/user/service.rs

// dependencies
use super::dto::{
    ChangePasswordRequest, CreateUserRequest, LoginRequest, UpdateUserRequest, UserResponse,
    UserSummary,
};
use super::error::UserError;
use super::repository::UserRepository;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;
use validator::Validate;

// traits
#[async_trait]
pub trait UserService: Send + Sync {
    async fn register(&self, request: CreateUserRequest) -> Result<UserResponse, UserError>;
    async fn login(&self, request: LoginRequest) -> Result<UserSummary, UserError>;
    async fn get_user(&self, id: Uuid) -> Result<UserResponse, UserError>;
    async fn get_user_summary(&self, id: Uuid) -> Result<UserSummary, UserError>;
    async fn update_profile(
        &self,
        id: Uuid,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, UserError>;
    async fn change_password(
        &self,
        id: Uuid,
        request: ChangePasswordRequest,
    ) -> Result<(), UserError>;
    // New: email verification related operations
    async fn set_verification_token(&self, id: Uuid) -> Result<String, UserError>;
    async fn verify_email(&self, token: &str) -> Result<bool, UserError>;
    async fn resend_verification(&self, email: &str) -> Result<(), UserError>;
}

pub struct UserServiceImpl {
    repository: Arc<dyn UserRepository>,
    resend_tracker: Mutex<HashMap<String, Instant>>, // in-memory rate limiter (per-process)
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self {
            repository,
            resend_tracker: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn register(&self, request: CreateUserRequest) -> Result<UserResponse, UserError> {
        // Validate input
        request.validate().map_err(|e| UserError::Validation {
            message: format!("Validation failed: {e}"),
        })?;

        // Create user
        let user = self.repository.create(request).await?;

        Ok(UserResponse::from(user))
    }

    async fn login(&self, request: LoginRequest) -> Result<UserSummary, UserError> {
        // Validate input
        request.validate().map_err(|e| UserError::Validation {
            message: format!("Validation failed: {e}"),
        })?;

        // Find user by username or email
        let user = self
            .repository
            .find_by_username_or_email(&request.username_or_email)
            .await?
            .ok_or(UserError::InvalidCredentials)?;

        // Verify password
        let is_valid = self
            .repository
            .verify_password(&user, &request.password)
            .await?;
        if !is_valid {
            return Err(UserError::InvalidCredentials);
        }

        // Check if user is active and verified
        if !user.is_active || !user.email_verified {
            return Err(UserError::InvalidCredentials);
        }

        Ok(UserSummary {
            id: user.id,
            username: user.username,
            display_name: None,
            avatar_url: None,
            role: user.role,
        })
    }

    async fn get_user(&self, id: Uuid) -> Result<UserResponse, UserError> {
        let user = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(UserError::UserNotFound)?;

        Ok(UserResponse::from(user))
    }

    async fn get_user_summary(&self, id: Uuid) -> Result<UserSummary, UserError> {
        let user = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(UserError::UserNotFound)?;

        Ok(UserSummary::from(user))
    }

    async fn update_profile(
        &self,
        id: Uuid,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, UserError> {
        // Validate input
        request.validate().map_err(|e| UserError::Validation {
            message: format!("Validation failed: {e}"),
        })?;

        // Update user
        let user = self.repository.update(id, request).await?;

        Ok(UserResponse::from(user))
    }

    async fn change_password(
        &self,
        id: Uuid,
        request: ChangePasswordRequest,
    ) -> Result<(), UserError> {
        // Validate input
        request.validate().map_err(|e| UserError::Validation {
            message: format!("Validation failed: {e}"),
        })?;

        // Get current user
        let user = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(UserError::UserNotFound)?;

        // Verify current password
        let is_valid = self
            .repository
            .verify_password(&user, &request.current_password)
            .await?;
        if !is_valid {
            return Err(UserError::InvalidCredentials);
        }

        // Change password
        self.repository
            .change_password(id, &request.new_password)
            .await?;

        Ok(())
    }

    async fn set_verification_token(&self, id: Uuid) -> Result<String, UserError> {
        let token = Uuid::new_v4().to_string();
        self.repository
            .set_email_verification_token(id, token.clone())
            .await?;
        Ok(token)
    }

    async fn verify_email(&self, token: &str) -> Result<bool, UserError> {
        let user = self.repository.verify_email(token).await?;
        Ok(user.is_some())
    }

    async fn resend_verification(&self, email: &str) -> Result<(), UserError> {
        // In-memory rate-limit: block re-requests within 60 seconds
        {
            let mut guard = self
                .resend_tracker
                .lock()
                .expect("resend_tracker mutex poisoned");
            let key = email.to_lowercase();
            if let Some(last) = guard.get(&key) {
                if last.elapsed() < Duration::from_secs(60) {
                    return Err(UserError::Validation {
                        message: "Please wait before requesting another verification email".into(),
                    });
                }
            }
            guard.insert(key, Instant::now());
        }

        if let Some(user) = self.repository.find_by_email(email).await? {
            if user.email_verified {
                return Ok(());
            }
            // Always issue a new token on resend
            let _ = self.set_verification_token(user.id).await?;
        }
        // Do not reveal if user exists
        Ok(())
    }
}
