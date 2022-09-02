pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// A URL could not be parsed and thus is invalid
    UrlParse(url::ParseError),
    /// The actual request failed, mostly due to network erros
    Reqwest(reqwest::Error),
    /// The Smarthome server responded with an unexpected status code
    Smarthome(reqwest::StatusCode),
    /// A semantiv version number could not be parsed and thus is invalid
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
