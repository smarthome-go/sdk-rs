mod client;

mod auth;
mod errors;
mod request;
mod version;

mod homescript;

pub use auth::{Auth, User};
pub use client::Client;

pub use homescript::exec::*;
pub use homescript::*;

const SERVER_VERSION_REQUIREMENT: &str = "^0.2.0";
const HTTP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
