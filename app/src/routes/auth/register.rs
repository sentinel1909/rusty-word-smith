// app/src/routes/auth/register.rs

// dependencies
use crate::errors::ApiError;
use crate::models::{CreateUserRequest, UserResponse};
use crate::models::UserService;
use pavex::{post, methods, request::body::JsonBody};
use std::sync::Arc;
use sqlx::PgPool;
use crate::models::SqlxUserRepository;
use crate::models::UserServiceImpl;

pub struct UserServiceContainer(pub Box<dyn UserService>);

#[methods]
impl UserServiceContainer {
    #[singleton]
    pub fn new(pool: &PgPool) -> Self {
        let repository = Arc::new(SqlxUserRepository::new(pool.clone()));
        let service = UserServiceImpl::new(repository);
        UserServiceContainer(Box::new(service))
    }
}

// handler which will be called when the user visits the register page
#[post(path = "/auth/register")]
pub async fn register(
    body: &JsonBody<CreateUserRequest>,
    user_service: &UserServiceContainer,
) -> Result<UserResponse, ApiError> {
    let create_user_request = body.0.clone();
    
    let user_response = user_service
        .0
        .register(create_user_request)
        .await?;
    
    Ok(user_response)
}
