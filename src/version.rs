use crate::{errors::Result, Client, SERVER_VERSION_REQUIREMENT};
use semver::{Version, VersionReq};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VersionResponse {
    #[serde(rename = "version")]
    pub smarthome_version: String,
    #[serde(rename = "goVersion")]
    pub go_version: String,
}

pub fn is_server_compatible(server_version: &str) -> Result<bool> {
    let req = VersionReq::parse(SERVER_VERSION_REQUIREMENT)?;
    let mut version = Version::parse(server_version)?;

    // `VersionReq::matches` rejects pre-release versions (e.g. `0.15.2-alpha`) unless the
    // requirement explicitly opts into pre-releases, so compare against the release part only.
    version.pre = semver::Prerelease::EMPTY;

    Ok(req.matches(&version))
}

impl Client {
    pub fn smarthome_version(&self) -> &VersionResponse {
        &self.smarthome_version
    }
}

#[cfg(test)]
mod tests {
    use super::is_server_compatible;

    #[test]
    fn prerelease_server_is_compatible() {
        assert!(is_server_compatible("0.15.2-alpha").unwrap());
        assert!(is_server_compatible("0.4.0").unwrap());
        assert!(!is_server_compatible("0.3.9-beta").unwrap());
    }
}
