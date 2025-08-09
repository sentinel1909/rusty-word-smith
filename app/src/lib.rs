// app/src/lib.rs

// modules
pub mod authorization;
mod blueprint;
pub mod configuration;
pub mod errors;
pub mod models;
pub mod response;
pub mod routes;
pub mod telemetry;

// re-export the blueprint
pub use blueprint::blueprint;
