use crate::{errors::Result, SERVER_VERSION_REQUIREMENT};
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
