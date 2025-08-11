// app/src/models/user/repository.rs

// dependencies
use super::dto::{CreateUserRequest, UpdateUserRequest};
use super::entity::{User, UserRole};
use super::error::UserError;
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;
use jiff_sqlx::Timestamp as SqlxTimestamp;
use pavex::time::Timestamp;
use sqlx::{PgPool, Row};
use std::time::Duration;
use uuid::Uuid;

// traits
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, request: CreateUserRequest) -> Result<User, UserError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError>;
    async fn find_by_username_or_email(
        &self,
        username_or_email: &str,
    ) -> Result<Option<User>, UserError>;
    async fn update(&self, id: Uuid, request: UpdateUserRequest) -> Result<User, UserError>;
    async fn verify_password(&self, user: &User, password: &str) -> Result<bool, UserError>;
    async fn change_password(&self, id: Uuid, new_password: &str) -> Result<(), UserError>;
    async fn set_email_verification_token(&self, id: Uuid, token: String) -> Result<(), UserError>;
    async fn verify_email(&self, token: &str) -> Result<Option<User>, UserError>;
    async fn set_password_reset_token(&self, id: Uuid, token: String) -> Result<(), UserError>;
    async fn reset_password(
        &self,
        token: &str,
        new_password: &str,
    ) -> Result<Option<User>, UserError>;
}

pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Hash a password using Argon2
    fn hash_password(password: &str) -> Result<String, UserError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| UserError::PasswordHash(e.to_string()))
    }

    /// Verify a password against a hash
    fn verify_password_hash(password: &str, hash: &str) -> Result<bool, UserError> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| UserError::PasswordHash(e.to_string()))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Helper function to map database row to User struct
    fn map_row_to_user(row: sqlx::postgres::PgRow) -> Result<User, UserError> {
        let role: UserRole = row.get("role");
        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            display_name: row.get("display_name"),
            bio: row.get("bio"),
            avatar_url: row.get("avatar_url"),
            role,
            is_active: row.get("is_active"),
            email_verified: row.get("email_verified"),
            email_verification_token: row.get("email_verification_token"),

            email_verification_expires_at: row
                .get::<Option<SqlxTimestamp>, _>("email_verification_expires_at")
                .map(|t| t.into()),
            password_reset_token: row.get("password_reset_token"),
            password_reset_expires_at: row
                .get::<Option<SqlxTimestamp>, _>("password_reset_expires_at")
                .map(|t| t.into()),
            social_twitter: row.get("social_twitter"),
            social_github: row.get("social_github"),
            website_url: row.get("website_url"),
            created_at: row.get::<SqlxTimestamp, _>("created_at").into(),
            updated_at: row.get::<SqlxTimestamp, _>("updated_at").into(),
        })
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn create(&self, request: CreateUserRequest) -> Result<User, UserError> {
        // Check if username already exists
        if (self.find_by_username(&request.username).await?).is_some() {
            return Err(UserError::UsernameExists);
        }

        // Check if email already exists
        if (self.find_by_email(&request.email).await?).is_some() {
            return Err(UserError::EmailExists);
        }

        // Hash the password
        let password_hash = Self::hash_password(&request.password)?;

        // Insert new user with manual query
        let row = sqlx::query(
            r#"
            INSERT INTO users (username, email, password_hash, display_name)
            VALUES ($1, $2, $3, $4)
            RETURNING 
                id, username, email, password_hash, display_name, bio, avatar_url,
                role, is_active, email_verified,
                email_verification_token, email_verification_expires_at,
                password_reset_token, password_reset_expires_at,
                social_twitter, social_github, website_url,
                created_at, updated_at
            "#,
        )
        .bind(&request.username)
        .bind(&request.email)
        .bind(&password_hash)
        .bind(&request.display_name)
        .fetch_one(&self.pool)
        .await?;

        Self::map_row_to_user(row)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, display_name, bio, avatar_url,
                role, is_active, email_verified,
                email_verification_token, email_verification_expires_at,
                password_reset_token, password_reset_expires_at,
                social_twitter, social_github, website_url,
                created_at, updated_at
            FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Self::map_row_to_user(row).map(Some),
            None => Ok(None),
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserError> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, display_name, bio, avatar_url,
                role, is_active, email_verified,
                email_verification_token, email_verification_expires_at,
                password_reset_token, password_reset_expires_at,
                social_twitter, social_github, website_url,
                created_at, updated_at
            FROM users WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Self::map_row_to_user(row).map(Some),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, display_name, bio, avatar_url,
                role, is_active, email_verified,
                email_verification_token, email_verification_expires_at,
                password_reset_token, password_reset_expires_at,
                social_twitter, social_github, website_url,
                created_at, updated_at
            FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Self::map_row_to_user(row).map(Some),
            None => Ok(None),
        }
    }

    async fn find_by_username_or_email(
        &self,
        username_or_email: &str,
    ) -> Result<Option<User>, UserError> {
        let row = sqlx::query(
            r#"
            SELECT 
                id, username, email, password_hash, display_name, bio, avatar_url,
                role, is_active, email_verified,
                email_verification_token, email_verification_expires_at,
                password_reset_token, password_reset_expires_at,
                social_twitter, social_github, website_url,
                created_at, updated_at
            FROM users 
            WHERE username = $1 OR email = $1
            "#,
        )
        .bind(username_or_email)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Self::map_row_to_user(row).map(Some),
            None => Ok(None),
        }
    }

    async fn update(&self, id: Uuid, request: UpdateUserRequest) -> Result<User, UserError> {
        let row = sqlx::query(
            r#"
            UPDATE users 
            SET 
                display_name = COALESCE($2, display_name),
                bio = COALESCE($3, bio),
                avatar_url = COALESCE($4, avatar_url),
                social_twitter = COALESCE($5, social_twitter),
                social_github = COALESCE($6, social_github),
                website_url = COALESCE($7, website_url),
                updated_at = NOW()
            WHERE id = $1
            RETURNING 
                id, username, email, password_hash, display_name, bio, avatar_url,
                role, is_active, email_verified,
                email_verification_token, email_verification_expires_at,
                password_reset_token, password_reset_expires_at,
                social_twitter, social_github, website_url,
                created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&request.display_name)
        .bind(&request.bio)
        .bind(&request.avatar_url)
        .bind(&request.social_twitter)
        .bind(&request.social_github)
        .bind(&request.website_url)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(UserError::UserNotFound)?;

        Self::map_row_to_user(row)
    }

    async fn verify_password(&self, user: &User, password: &str) -> Result<bool, UserError> {
        Self::verify_password_hash(password, &user.password_hash)
    }

    async fn change_password(&self, id: Uuid, new_password: &str) -> Result<(), UserError> {
        let password_hash = Self::hash_password(new_password)?;

        let result =
            sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2")
                .bind(&password_hash)
                .bind(id)
                .execute(&self.pool)
                .await?;

        if result.rows_affected() == 0 {
            return Err(UserError::UserNotFound);
        }

        Ok(())
    }

    async fn set_email_verification_token(&self, id: Uuid, token: String) -> Result<(), UserError> {
        let expires_at = Timestamp::now() + Duration::from_secs(24 * 60 * 60); // 24 hours

        let result = sqlx::query(
            "UPDATE users SET email_verification_token = $1, email_verification_expires_at = $2 WHERE id = $3"
        )
        .bind(&token)
        .bind(SqlxTimestamp::from(expires_at))
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(UserError::UserNotFound);
        }

        Ok(())
    }

    async fn verify_email(&self, token: &str) -> Result<Option<User>, UserError> {
        let row = sqlx::query(
            r#"
            UPDATE users 
            SET 
                email_verified = true,
                email_verification_token = NULL,
                email_verification_expires_at = NULL,
                updated_at = NOW()
            WHERE email_verification_token = $1 
            AND email_verification_expires_at > NOW()
            RETURNING 
                id, username, email, password_hash, display_name, bio, avatar_url,
                role, is_active, email_verified,
                email_verification_token, email_verification_expires_at,
                password_reset_token, password_reset_expires_at,
                social_twitter, social_github, website_url,
                created_at, updated_at
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Self::map_row_to_user(row).map(Some),
            None => Ok(None),
        }
    }

    async fn set_password_reset_token(&self, id: Uuid, token: String) -> Result<(), UserError> {
        let expires_at = Timestamp::now() + Duration::from_secs(60 * 60); // 1 hour

        let result = sqlx::query(
            "UPDATE users SET password_reset_token = $1, password_reset_expires_at = $2 WHERE id = $3"
        )
        .bind(&token)
        .bind(expires_at.to_string()) // Convert to string for SQL
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(UserError::UserNotFound);
        }

        Ok(())
    }

    async fn reset_password(
        &self,
        token: &str,
        new_password: &str,
    ) -> Result<Option<User>, UserError> {
        let password_hash = Self::hash_password(new_password)?;

        let row = sqlx::query(
            r#"
            UPDATE users 
            SET 
                password_hash = $1,
                password_reset_token = NULL,
                password_reset_expires_at = NULL,
                updated_at = NOW()
            WHERE password_reset_token = $2 
            AND password_reset_expires_at > NOW()
            RETURNING 
                id, username, email, password_hash, display_name, bio, avatar_url,
                role, is_active, email_verified,
                email_verification_token, email_verification_expires_at,
                password_reset_token, password_reset_expires_at,
                social_twitter, social_github, website_url,
                created_at, updated_at
            "#,
        )
        .bind(&password_hash)
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Self::map_row_to_user(row).map(Some),
            None => Ok(None),
        }
    }
}
