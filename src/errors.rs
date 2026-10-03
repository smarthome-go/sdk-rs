use std::fmt::Display;

use reqwest::StatusCode;
use serde::Deserialize;

use crate::SERVER_VERSION_REQUIREMENT;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// A URL could not be parsed and thus is invalid
    UrlParse(url::ParseError),
    /// The actual request failed, mostly due to network errors
    Reqwest(reqwest::Error),
    /// The Smarthome server responded with an unexpected status code
    Smarthome(reqwest::StatusCode),
    /// The Smarthome server responded with an unexpected status code and explained why
    SmarthomeResponse {
        status: reqwest::StatusCode,
        message: String,
        error: String,
    },
    /// A semantic version number could not be parsed and thus is invalid
    VersionParse(semver::Error),
    /// The SDK cannot connect to a Server which is incompatible
    IncompatibleVersion(String),
}

/// The JSON body the Smarthome server sends along with error status codes
#[derive(Deserialize)]
struct ErrorResponse {
    message: String,
    error: String,
}

impl Error {
    /// Builds an error from a non-OK response, keeping the server's explanation if it sent one
    pub(crate) async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        match response.json::<ErrorResponse>().await {
            Ok(body) => Self::SmarthomeResponse {
                status,
                message: body.message,
                error: body.error,
            },
            Err(_) => Self::Smarthome(status),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(mut err: reqwest::Error) -> Self {
        // Request URLs carry the credentials in their query, which must not end up in messages
        if let Some(url) = err.url_mut() {
            url.set_query(None);
        }
        Self::Reqwest(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParse(err)
    }
}

impl From<semver::Error> for Error {
    fn from(err: semver::Error) -> Self {
        Self::VersionParse(err)
    }
}

impl Error {
    /// The HTTP status code of the server's response, if this error was caused by one
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Smarthome(status) | Self::SmarthomeResponse { status, .. } => Some(*status),
            _ => None,
        }
    }
}

/// A hint on how to resolve status codes whose cause is usually the same
fn status_hint(status: StatusCode) -> Option<&'static str> {
    match status {
        StatusCode::UNAUTHORIZED => {
            Some("Login failed: validate your username + password or access token")
        }
        StatusCode::FORBIDDEN => {
            Some("You are possibly lacking permission to access the requested resource")
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            Some("The server has significant issues and was unable to respond properly")
        }
        StatusCode::CONFLICT => {
            Some("The requested action conflicts with other data on the system")
        }
        _ => None,
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UrlParse(err) => write!(f, "could not parse URL: {err}"),
            Self::Reqwest(err) => {
                // `reqwest` keeps the actual cause (e.g. a missing JSON field) in its sources
                write!(f, "request failed: {err}")?;
                let mut source = std::error::Error::source(err);
                while let Some(cause) = source {
                    write!(f, ": {cause}")?;
                    source = cause.source();
                }
                Ok(())
            }
            Self::Smarthome(status) => write!(f, "server responded with {status}"),
            Self::SmarthomeResponse {
                status,
                message,
                error,
            } => {
                write!(f, "server responded with {status}: {message}")?;
                match error.is_empty() || error == message {
                    true => Ok(()),
                    false => write!(f, ": {error}"),
                }
            }
            Self::VersionParse(err) => write!(f, "could not parse the server's version: {err}"),
            Self::IncompatibleVersion(server_version) => write!(
                f,
                "incompatible server version: the server runs `{server_version}` but this program requires `{SERVER_VERSION_REQUIREMENT}`"
            ),
        }?;

        match self.status().and_then(status_hint) {
            Some(hint) => write!(f, "\n => {hint}"),
            None => Ok(()),
        }
    }
}

// `Display` already includes the underlying errors, so `source` is left empty to avoid
// printing them twice in error chains (e.g. `anyhow`'s `{:#}`)
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    fn response(status: u16, body: &str) -> reqwest::Response {
        http::Response::builder()
            .status(status)
            .body(body.to_string())
            .unwrap()
            .into()
    }

    #[tokio::test]
    async fn keeps_the_servers_explanation() {
        let err = Error::from_response(response(
            422,
            r#"{"success":false,"message":"Validation failed","error":"unknown variable `foo`","time":""}"#,
        ))
        .await;

        assert_eq!(err.status(), Some(StatusCode::UNPROCESSABLE_ENTITY));
        assert_eq!(
            err.to_string(),
            "server responded with 422 Unprocessable Entity: Validation failed: unknown variable `foo`"
        );
    }

    #[tokio::test]
    async fn falls_back_to_the_status_code() {
        let err = Error::from_response(response(401, "not json")).await;

        assert!(matches!(err, Error::Smarthome(StatusCode::UNAUTHORIZED)));
        assert_eq!(
            err.to_string(),
            "server responded with 401 Unauthorized\n => Login failed: validate your username + password or access token"
        );
    }

    #[tokio::test]
    async fn request_errors_do_not_leak_credentials() {
        // Nothing listens on port 1, so the request fails with the URL attached
        let err: Error = reqwest::get("http://127.0.0.1:1/api/debug?token=secret-token")
            .await
            .unwrap_err()
            .into();

        let message = err.to_string();
        assert!(
            message.contains("http://127.0.0.1:1/api/debug"),
            "{message}"
        );
        assert!(!message.contains("secret-token"), "{message}");
    }

    #[tokio::test]
    async fn does_not_repeat_identical_message_and_error() {
        let err = Error::from_response(response(
            400,
            r#"{"success":false,"message":"bad request","error":"bad request"}"#,
        ))
        .await;

        assert_eq!(
            err.to_string(),
            "server responded with 400 Bad Request: bad request"
        );
    }
}
