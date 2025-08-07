// app/src/models/user/tests.rs

#[cfg(test)]
mod tests {
    use crate::models::user::*;
    use async_trait::async_trait;
    use pavex::time::Timestamp;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;
    use validator::Validate;

    // Mock repository for testing the service layer
    pub struct MockUserRepository {
        users: Arc<Mutex<HashMap<Uuid, User>>>,
        users_by_username: Arc<Mutex<HashMap<String, Uuid>>>,
        users_by_email: Arc<Mutex<HashMap<String, Uuid>>>,
        password_verifications: Arc<Mutex<HashMap<Uuid, String>>>,
    }

    impl MockUserRepository {
        pub fn new() -> Self {
            Self {
                users: Arc::new(Mutex::new(HashMap::new())),
                users_by_username: Arc::new(Mutex::new(HashMap::new())),
                users_by_email: Arc::new(Mutex::new(HashMap::new())),
                password_verifications: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn insert_user(&self, user: User) {
            let mut users = self.users.lock().unwrap();
            let mut by_username = self.users_by_username.lock().unwrap();
            let mut by_email = self.users_by_email.lock().unwrap();

            by_username.insert(user.username.clone(), user.id);
            by_email.insert(user.email.clone(), user.id);
            users.insert(user.id, user);
        }

        pub fn set_password_verification(&self, user_id: Uuid, password: String) {
            let mut verifications = self.password_verifications.lock().unwrap();
            verifications.insert(user_id, password);
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, request: CreateUserRequest) -> Result<User, UserError> {
            let users = self.users.lock().unwrap();
            let by_username = self.users_by_username.lock().unwrap();
            let by_email = self.users_by_email.lock().unwrap();

            // Check for existing username
            if by_username.contains_key(&request.username) {
                return Err(UserError::UsernameExists);
            }

            // Check for existing email
            if by_email.contains_key(&request.email) {
                return Err(UserError::EmailExists);
            }

            drop(users);
            drop(by_username);
            drop(by_email);

            let user = User {
                id: Uuid::new_v4(),
                username: request.username,
                email: request.email,
                password_hash: "hashed_password".to_string(), // Mock hash
                display_name: request.display_name,
                bio: None,
                avatar_url: None,
                role: UserRole::Subscriber,
                is_active: true,
                email_verified: false,
                email_verification_token: None,
                email_verification_expires_at: None,
                password_reset_token: None,
                password_reset_expires_at: None,
                social_twitter: None,
                social_github: None,
                website_url: None,
                created_at: Timestamp::now(),
                updated_at: Timestamp::now(),
            };

            self.insert_user(user.clone());
            Ok(user)
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
            let users = self.users.lock().unwrap();
            Ok(users.get(&id).cloned())
        }

        async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserError> {
            let by_username = self.users_by_username.lock().unwrap();
            let users = self.users.lock().unwrap();

            if let Some(user_id) = by_username.get(username) {
                Ok(users.get(user_id).cloned())
            } else {
                Ok(None)
            }
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
            let by_email = self.users_by_email.lock().unwrap();
            let users = self.users.lock().unwrap();

            if let Some(user_id) = by_email.get(email) {
                Ok(users.get(user_id).cloned())
            } else {
                Ok(None)
            }
        }

        async fn find_by_username_or_email(
            &self,
            username_or_email: &str,
        ) -> Result<Option<User>, UserError> {
            // Try username first
            if let Some(user) = self.find_by_username(username_or_email).await? {
                return Ok(Some(user));
            }
            // Then try email
            self.find_by_email(username_or_email).await
        }

        async fn update(&self, id: Uuid, request: UpdateUserRequest) -> Result<User, UserError> {
            let mut users = self.users.lock().unwrap();

            if let Some(user) = users.get_mut(&id) {
                if let Some(display_name) = request.display_name {
                    user.display_name = Some(display_name);
                }
                if let Some(bio) = request.bio {
                    user.bio = Some(bio);
                }
                if let Some(avatar_url) = request.avatar_url {
                    user.avatar_url = Some(avatar_url);
                }
                if let Some(social_twitter) = request.social_twitter {
                    user.social_twitter = Some(social_twitter);
                }
                if let Some(social_github) = request.social_github {
                    user.social_github = Some(social_github);
                }
                if let Some(website_url) = request.website_url {
                    user.website_url = Some(website_url);
                }
                user.updated_at = Timestamp::now();
                Ok(user.clone())
            } else {
                Err(UserError::UserNotFound)
            }
        }

        async fn verify_password(&self, user: &User, password: &str) -> Result<bool, UserError> {
            let verifications = self.password_verifications.lock().unwrap();
            if let Some(stored_password) = verifications.get(&user.id) {
                Ok(stored_password == password)
            } else {
                Ok(password == "correct_password") // Default mock behavior
            }
        }

        async fn change_password(&self, id: Uuid, new_password: &str) -> Result<(), UserError> {
            let users = self.users.lock().unwrap();
            if users.contains_key(&id) {
                let mut verifications = self.password_verifications.lock().unwrap();
                verifications.insert(id, new_password.to_string());
                Ok(())
            } else {
                Err(UserError::UserNotFound)
            }
        }

        async fn set_email_verification_token(
            &self,
            _id: Uuid,
            _token: String,
        ) -> Result<(), UserError> {
            Ok(()) // Mock implementation
        }

        async fn verify_email(&self, _token: &str) -> Result<Option<User>, UserError> {
            Ok(None) // Mock implementation
        }

        async fn set_password_reset_token(
            &self,
            _id: Uuid,
            _token: String,
        ) -> Result<(), UserError> {
            Ok(()) // Mock implementation
        }

        async fn reset_password(
            &self,
            _token: &str,
            _new_password: &str,
        ) -> Result<Option<User>, UserError> {
            Ok(None) // Mock implementation
        }
    }

    // Test helper functions
    fn create_valid_user_request() -> CreateUserRequest {
        CreateUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: Some("Test User".to_string()),
        }
    }

    fn create_valid_login_request() -> LoginRequest {
        LoginRequest {
            username_or_email: "testuser".to_string(),
            password: "correct_password".to_string(),
        }
    }

    // Entity tests
    #[test]
    fn test_user_role_from_str() {
        assert_eq!(UserRole::frm_str("admin"), Some(UserRole::Admin));
        assert_eq!(UserRole::frm_str("editor"), Some(UserRole::Editor));
        assert_eq!(UserRole::frm_str("author"), Some(UserRole::Author));
        assert_eq!(
            UserRole::frm_str("contributor"),
            Some(UserRole::Contributor)
        );
        assert_eq!(UserRole::frm_str("subscriber"), Some(UserRole::Subscriber));
        assert_eq!(UserRole::frm_str("invalid"), None);
    }

    #[test]
    fn test_user_permissions() {
        let admin_user = User {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            email: "admin@test.com".to_string(),
            password_hash: "hash".to_string(),
            display_name: None,
            bio: None,
            avatar_url: None,
            role: UserRole::Admin,
            is_active: true,
            email_verified: true,
            email_verification_token: None,
            email_verification_expires_at: None,
            password_reset_token: None,
            password_reset_expires_at: None,
            social_twitter: None,
            social_github: None,
            website_url: None,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
        };

        assert!(admin_user.is_admin());
        assert!(admin_user.can_write());
        assert!(admin_user.can_moderate());

        let subscriber_user = User {
            role: UserRole::Subscriber,
            ..admin_user.clone()
        };

        assert!(!subscriber_user.is_admin());
        assert!(!subscriber_user.can_write());
        assert!(!subscriber_user.can_moderate());
    }

    #[test]
    fn test_user_display_name() {
        let user_with_display_name = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            display_name: Some("Test User".to_string()),
            bio: None,
            avatar_url: None,
            role: UserRole::Subscriber,
            is_active: true,
            email_verified: false,
            email_verification_token: None,
            email_verification_expires_at: None,
            password_reset_token: None,
            password_reset_expires_at: None,
            social_twitter: None,
            social_github: None,
            website_url: None,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
        };

        assert_eq!(user_with_display_name.display_name(), "Test User");

        let user_without_display_name = User {
            display_name: None,
            ..user_with_display_name
        };

        assert_eq!(user_without_display_name.display_name(), "testuser");
    }

    // DTO validation tests
    #[test]
    fn test_create_user_request_validation() {
        // Valid request
        let valid_request = create_valid_user_request();
        assert!(valid_request.validate().is_ok());

        // Invalid username (too short)
        let invalid_username = CreateUserRequest {
            username: "ab".to_string(),
            ..valid_request.clone()
        };
        assert!(invalid_username.validate().is_err());

        // Invalid email
        let invalid_email = CreateUserRequest {
            email: "not_an_email".to_string(),
            ..valid_request.clone()
        };
        assert!(invalid_email.validate().is_err());

        // Invalid password (too short)
        let invalid_password = CreateUserRequest {
            password: "short".to_string(),
            ..valid_request.clone()
        };
        assert!(invalid_password.validate().is_err());
    }

    #[test]
    fn test_login_request_validation() {
        let valid_request = create_valid_login_request();
        assert!(valid_request.validate().is_ok());

        // Empty username/email
        let invalid_request = LoginRequest {
            username_or_email: "".to_string(),
            ..valid_request.clone()
        };
        assert!(invalid_request.validate().is_err());

        // Empty password
        let invalid_request = LoginRequest {
            password: "".to_string(),
            ..valid_request.clone()
        };
        assert!(invalid_request.validate().is_err());
    }

    // Repository tests
    #[tokio::test]
    async fn test_repository_create_user_success() {
        let repo = MockUserRepository::new();
        let request = create_valid_user_request();

        let result = repo.create(request.clone()).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.username, request.username);
        assert_eq!(user.email, request.email);
        assert_eq!(user.display_name, request.display_name);
        assert_eq!(user.role, UserRole::Subscriber);
        assert!(user.is_active);
        assert!(!user.email_verified);
    }

    #[tokio::test]
    async fn test_repository_create_user_duplicate_username() {
        let repo = MockUserRepository::new();
        let request = create_valid_user_request();

        // Create first user
        let _first_user = repo.create(request.clone()).await.unwrap();

        // Try to create second user with same username
        let second_request = CreateUserRequest {
            email: "different@example.com".to_string(),
            ..request
        };

        let result = repo.create(second_request).await;
        assert!(matches!(result, Err(UserError::UsernameExists)));
    }

    #[tokio::test]
    async fn test_repository_create_user_duplicate_email() {
        let repo = MockUserRepository::new();
        let request = create_valid_user_request();

        // Create first user
        let _first_user = repo.create(request.clone()).await.unwrap();

        // Try to create second user with same email
        let second_request = CreateUserRequest {
            username: "different_user".to_string(),
            ..request
        };

        let result = repo.create(second_request).await;
        assert!(matches!(result, Err(UserError::EmailExists)));
    }

    #[tokio::test]
    async fn test_repository_find_by_username() {
        let repo = MockUserRepository::new();
        let request = create_valid_user_request();
        let created_user = repo.create(request).await.unwrap();

        let found_user = repo.find_by_username("testuser").await.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().id, created_user.id);

        let not_found = repo.find_by_username("nonexistent").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_repository_find_by_email() {
        let repo = MockUserRepository::new();
        let request = create_valid_user_request();
        let created_user = repo.create(request).await.unwrap();

        let found_user = repo.find_by_email("test@example.com").await.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().id, created_user.id);

        let not_found = repo.find_by_email("nonexistent@example.com").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_repository_update_user() {
        let repo = MockUserRepository::new();
        let request = create_valid_user_request();
        let created_user = repo.create(request).await.unwrap();

        let update_request = UpdateUserRequest {
            display_name: Some("Updated Name".to_string()),
            bio: Some("Updated bio".to_string()),
            avatar_url: None,
            social_twitter: Some("@updated".to_string()),
            social_github: Some("updated_user".to_string()),
            website_url: Some("https://updated.com".to_string()),
        };

        let updated_user = repo.update(created_user.id, update_request).await.unwrap();
        assert_eq!(updated_user.display_name, Some("Updated Name".to_string()));
        assert_eq!(updated_user.bio, Some("Updated bio".to_string()));
        assert_eq!(updated_user.social_twitter, Some("@updated".to_string()));
        assert_eq!(updated_user.social_github, Some("updated_user".to_string()));
        assert_eq!(
            updated_user.website_url,
            Some("https://updated.com".to_string())
        );
    }

    #[tokio::test]
    async fn test_repository_update_nonexistent_user() {
        let repo = MockUserRepository::new();
        let update_request = UpdateUserRequest {
            display_name: Some("Updated Name".to_string()),
            bio: None,
            avatar_url: None,
            social_twitter: None,
            social_github: None,
            website_url: None,
        };

        let result = repo.update(Uuid::new_v4(), update_request).await;
        assert!(matches!(result, Err(UserError::UserNotFound)));
    }

    #[tokio::test]
    async fn test_repository_verify_password() {
        let repo = MockUserRepository::new();
        let request = create_valid_user_request();
        let created_user = repo.create(request).await.unwrap();

        // Set up password verification
        repo.set_password_verification(created_user.id, "correct_password".to_string());

        let is_valid = repo
            .verify_password(&created_user, "correct_password")
            .await
            .unwrap();
        assert!(is_valid);

        let is_invalid = repo
            .verify_password(&created_user, "wrong_password")
            .await
            .unwrap();
        assert!(!is_invalid);
    }

    // Service tests
    #[tokio::test]
    async fn test_service_register_success() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserServiceImpl::new(repo);
        let request = create_valid_user_request();

        let result = service.register(request.clone()).await;
        assert!(result.is_ok());

        let user_response = result.unwrap();
        assert_eq!(user_response.username, request.username);
        assert_eq!(user_response.email, request.email);
        assert_eq!(user_response.display_name, request.display_name);
    }

    #[tokio::test]
    async fn test_service_register_invalid_data() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserServiceImpl::new(repo);

        let invalid_request = CreateUserRequest {
            username: "ab".to_string(), // Too short
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        let result = service.register(invalid_request).await;
        assert!(matches!(result, Err(UserError::Validation { .. })));
    }

    #[tokio::test]
    async fn test_service_login_success() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserServiceImpl::new(repo.clone());

        // Create a user first
        let create_request = create_valid_user_request();
        let created_user = repo.create(create_request).await.unwrap();
        repo.set_password_verification(created_user.id, "correct_password".to_string());

        // Test login
        let login_request = create_valid_login_request();
        let result = service.login(login_request).await;
        assert!(result.is_ok());

        let user_response = result.unwrap();
        assert_eq!(user_response.username, created_user.username);
    }

    #[tokio::test]
    async fn test_service_login_invalid_credentials() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserServiceImpl::new(repo.clone());

        let login_request = LoginRequest {
            username_or_email: "nonexistent".to_string(),
            password: "password".to_string(),
        };

        let result = service.login(login_request).await;
        assert!(matches!(result, Err(UserError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_service_get_user() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserServiceImpl::new(repo.clone());

        // Create a user first
        let create_request = create_valid_user_request();
        let created_user = repo.create(create_request).await.unwrap();

        // Test get user
        let result = service.get_user(created_user.id).await;
        assert!(result.is_ok());

        let user_response = result.unwrap();
        assert_eq!(user_response.id, created_user.id);
        assert_eq!(user_response.username, created_user.username);
    }

    #[tokio::test]
    async fn test_service_get_nonexistent_user() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserServiceImpl::new(repo);

        let result = service.get_user(Uuid::new_v4()).await;
        assert!(matches!(result, Err(UserError::UserNotFound)));
    }

    #[tokio::test]
    async fn test_service_update_profile() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserServiceImpl::new(repo.clone());

        // Create a user first
        let create_request = create_valid_user_request();
        let created_user = repo.create(create_request).await.unwrap();

        // Update profile
        let update_request = UpdateUserRequest {
            display_name: Some("New Display Name".to_string()),
            bio: Some("New bio".to_string()),
            avatar_url: None,
            social_twitter: None,
            social_github: None,
            website_url: None,
        };

        let result = service
            .update_profile(created_user.id, update_request)
            .await;
        assert!(result.is_ok());

        let updated_response = result.unwrap();
        assert_eq!(
            updated_response.display_name,
            Some("New Display Name".to_string())
        );
        assert_eq!(updated_response.bio, Some("New bio".to_string()));
    }

    #[tokio::test]
    async fn test_service_change_password() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserServiceImpl::new(repo.clone());

        // Create a user first
        let create_request = create_valid_user_request();
        let created_user = repo.create(create_request).await.unwrap();
        repo.set_password_verification(created_user.id, "current_password".to_string());

        // Change password
        let change_request = ChangePasswordRequest {
            current_password: "current_password".to_string(),
            new_password: "new_password123".to_string(),
        };

        let result = service
            .change_password(created_user.id, change_request)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_service_change_password_wrong_current() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserServiceImpl::new(repo.clone());

        // Create a user first
        let create_request = create_valid_user_request();
        let created_user = repo.create(create_request).await.unwrap();
        repo.set_password_verification(created_user.id, "current_password".to_string());

        // Try to change password with wrong current password
        let change_request = ChangePasswordRequest {
            current_password: "wrong_password".to_string(),
            new_password: "new_password123".to_string(),
        };

        let result = service
            .change_password(created_user.id, change_request)
            .await;
        assert!(matches!(result, Err(UserError::InvalidCredentials)));
    }

    // DTO conversion tests
    #[test]
    fn test_user_to_user_response() {
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed".to_string(),
            display_name: Some("Test User".to_string()),
            bio: Some("Test bio".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            role: UserRole::Author,
            is_active: true,
            email_verified: true,
            email_verification_token: None,
            email_verification_expires_at: None,
            password_reset_token: None,
            password_reset_expires_at: None,
            social_twitter: Some("@testuser".to_string()),
            social_github: Some("testuser".to_string()),
            website_url: Some("https://testuser.com".to_string()),
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
        };

        let response: UserResponse = user.clone().into();

        assert_eq!(response.id, user.id);
        assert_eq!(response.username, user.username);
        assert_eq!(response.email, user.email);
        assert_eq!(response.display_name, user.display_name);
        assert_eq!(response.bio, user.bio);
        assert_eq!(response.avatar_url, user.avatar_url);
        assert_eq!(response.role, user.role);
        assert_eq!(response.is_active, user.is_active);
        assert_eq!(response.email_verified, user.email_verified);
        assert_eq!(response.social_twitter, user.social_twitter);
        assert_eq!(response.social_github, user.social_github);
        assert_eq!(response.website_url, user.website_url);
        assert_eq!(response.created_at, user.created_at);
    }

    #[test]
    fn test_user_to_user_summary() {
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed".to_string(),
            display_name: Some("Test User".to_string()),
            bio: None,
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            role: UserRole::Editor,
            is_active: true,
            email_verified: false,
            email_verification_token: None,
            email_verification_expires_at: None,
            password_reset_token: None,
            password_reset_expires_at: None,
            social_twitter: None,
            social_github: None,
            website_url: None,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
        };

        let summary: UserSummary = user.clone().into();

        assert_eq!(summary.id, user.id);
        assert_eq!(summary.username, user.username);
        assert_eq!(summary.display_name, user.display_name);
        assert_eq!(summary.avatar_url, user.avatar_url);
        assert_eq!(summary.role, user.role);
    }
}
