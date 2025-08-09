// app/src/authorization/mod.rs

// modules
pub mod current_user;
pub mod guards;

// re-exports
pub use current_user::*;
pub use guards::*;

// constants used in sessions
pub const USER_ID: &str = "user.id";
pub const USER_ROLE: &str = "user.role";
pub const USERNAME: &str = "user.username";