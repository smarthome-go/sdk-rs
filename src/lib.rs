mod client;
mod auth;
mod version;
mod errors;

mod homescript;

pub use auth::{Auth, User};
pub use client::Client;

const SERVER_VERSION_REQUIREMENT: &str = "^0.2.0";
const HTTP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
