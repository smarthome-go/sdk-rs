use crate::{
    errors::{Error, Result},
    version::{check_version, VersionResponse},
    Auth, HTTP_USER_AGENT,
};

pub struct Client {
    pub client: reqwest::Client,
    pub auth: Auth,
    pub smarthome_url: url::Url,
    pub smarthome_version: VersionResponse,
}

impl Client {
    pub async fn new(raw_url: &str, auth: Auth) -> Result<Self> {
        // Default client with user agent is created
        let client = reqwest::Client::builder()
            .user_agent(HTTP_USER_AGENT)
            .build()?;

        let smarthome_url = reqwest::Url::parse(&raw_url)?;

        // Fetch the version from the Smarthome server
        let mut version_url = smarthome_url.clone();
        version_url.set_path("/api/version");
        let res = reqwest::get(version_url).await?;

        // Handle errors which could occur during fetching
        if res.status() != reqwest::StatusCode::OK {
            return Err(Error::Smarthome(res.status()));
        }
        let version = res.json::<VersionResponse>().await?;

        // Check if the SDK's version is compatible with the server's version
        match check_version(&version.version) {
            Ok(true) => Ok(Self {
                client,
                auth,
                smarthome_url,
                smarthome_version: version,
            }),
            Ok(false) => Err(Error::IncompatibleVersion(version.version)),
            Err(err) => Err(err),
        }
    }
}
