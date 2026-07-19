use std::fmt::Display;

use reqwest::StatusCode;

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
    /// A semantic version number could not be parsed and thus is invalid
    VersionParse(semver::Error),
    /// The SDK cannot connect to a Server which is incompatible
    IncompatibleVersion(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
                    Error::UrlParse(err) =>
                        format!("Could not parse URL: {}", err),
                    Error::Reqwest(err) => format!("Request error: {err}"),
                    Error::Smarthome(status_code) => format!("Smarthome error ({status_code}):\n{}", match *status_code {
                        StatusCode::UNAUTHORIZED => "Login failed: invalid credentials\n => Validate your credentials",
                        StatusCode::FORBIDDEN => "Access to this resource has been denied.\n => You are possibly lacking permission to access the requested resource",
                        StatusCode::SERVICE_UNAVAILABLE => "Smarthome is currently unavailable\n => The server has significant issues and was unable to respond properly",
                        StatusCode::CONFLICT => "The requested action conflicts with other data on the system\n => Identify those conflicts and repeat the current action",
                        _ => "Unimplemented status code: please open an issue on Github here: (https://github.com/smarthome-go/sdk-rs)"
                    }),
                    Error::VersionParse(err) => panic!("Internal error: a version is invalid and could not be parsed: this is a bug and not your fault: {err}"),
                    Error::IncompatibleVersion(server_version) => format!("Incompatible server version: the server version is `{server_version}` but this program requires `{}`", SERVER_VERSION_REQUIREMENT)
        };
        write!(f, "{message}")
    }
}
