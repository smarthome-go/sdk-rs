mod client;

// Utility modules
mod auth;
mod errors;
mod request;
mod version;

// Functionality modules
mod homescript;
mod power;
mod room;

pub use auth::{Auth, User};
pub use client::Client;

// Re-exports
pub use errors::*;
pub use homescript::*;
pub use power::*;
pub use room::*;

/// This specifies the version constraints which are validated on a client's creation
pub const SERVER_VERSION_REQUIREMENT: &str = "^0.2.0";
const HTTP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
