mod client;

mod auth;
mod request;
mod errors;
mod version;

mod homescript;

pub use auth::{Auth, User};
pub use client::Client;

pub use homescript::*;
pub use homescript::exec::*;

const SERVER_VERSION_REQUIREMENT: &str = "^0.2.0";
const HTTP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
