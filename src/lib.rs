mod client;

mod auth;
mod errors;
mod request;
mod version;

mod homescript;
mod power;

pub use auth::{Auth, User};
pub use client::Client;

pub use homescript::exec::*;
pub use homescript::*;
pub use power::*;
pub use errors::*;

const SERVER_VERSION_REQUIREMENT: &str = "^0.2.0";
const HTTP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
