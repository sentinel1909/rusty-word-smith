// app/src/models/user/mod.rs

// modules
mod dto;
mod entity;
mod error;
mod repository;
mod service;

#[cfg(test)]
mod tests;

// re-export the modules
pub use dto::*;
pub use entity::*;
pub use error::*;
pub use repository::*;
pub use service::*;
