use crate::{errors::Result, Client, SERVER_VERSION_REQUIREMENT};
use semver::{Version, VersionReq};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VersionResponse {
    pub version: String,
    #[serde(rename = "goVersion")]
    pub go_version: String,
}

pub fn check_version(server_version: &str) -> Result<bool> {
    let req = VersionReq::parse(SERVER_VERSION_REQUIREMENT)?;
    let version = Version::parse(server_version)?;

    Ok(req.matches(&version))
}

impl Client {
    pub fn smarthome_version(&self) -> &VersionResponse {
        &self.smarthome_version
    }
}
