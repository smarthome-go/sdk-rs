pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UrlParse(url::ParseError),
    Reqwest(reqwest::Error),
    Smarthome(reqwest::StatusCode),
    VersionParse(semver::Error),
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
