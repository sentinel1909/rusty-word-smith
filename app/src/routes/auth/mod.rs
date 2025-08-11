// app/src/routes/auth/mod.rs

// modules
pub mod login;
pub mod logout;
pub mod verify;
pub mod resend;
pub mod check_email;
pub mod register;
pub mod whoami;

// re-exports
pub use login::*;
pub use logout::*;
pub use resend::*;
pub use check_email::*;
pub use register::*;
pub use verify::*;
pub use whoami::*;

// dependencies
use crate::models::{SqlxUserRepository, UserService, UserServiceImpl};
use pavex::methods;
use sqlx::PgPool;
use std::sync::Arc;

// struct type to wrap a user service in a container
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
