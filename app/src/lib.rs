// app/src/lib.rs

// modules
mod blueprint;
pub mod configuration;
pub mod errors;
pub mod routes;
pub mod telemetry;

// re-export the blueprint
pub use blueprint::blueprint;
