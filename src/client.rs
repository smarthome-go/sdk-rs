use reqwest::{Method, StatusCode};

use crate::{
    auth::Token,
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
        // Parse the source url
        let smarthome_url = reqwest::Url::parse(raw_url)?;
        // Default client with user agent is created
        let client = reqwest::Client::builder()
            .user_agent(HTTP_USER_AGENT)
            .build()?;

        // Fetch the current version from the Smarthome server
        let mut version_url = smarthome_url.clone();
        version_url.set_path("/api/version");
        let res = reqwest::get(version_url).await?;

        // Handle errors which could occur during fetching
        if res.status() != reqwest::StatusCode::OK {
            return Err(Error::Smarthome(res.status()));
        }
        let version = res.json::<VersionResponse>().await?;

        // Check if the SDK's version is compatible with the server's version
        let client = match check_version(&version.version) {
            Ok(true) => Self {
                client,
                auth,
                smarthome_url,
                smarthome_version: version,
            },
            Ok(false) => return Err(Error::IncompatibleVersion(version.version)),
            Err(err) => return Err(err),
        };

        // Attempt to login using the specified credentials
        match &client.auth {
            Auth::None => Ok(client),
            auth => match validate_credentials(&client.smarthome_url, auth).await {
                Ok(_) => Ok(client),
                Err(err) => Err(err),
            },
        }
    }
}

async fn validate_credentials(base_url: &url::Url, auth: &Auth) -> Result<()> {
    let mut login_url = base_url.clone();
    // Choose an adequate URL depending on the authentication mode
    login_url.set_path(match auth {
        Auth::QueryToken(_) => "/api/login/token",
        Auth::QueryPassword(_) => "/api/login",
        _ => unreachable!("login may not be called when using auth method `None`"),
    });

    // Perform the request
    let req = reqwest::Client::new().request(Method::POST, login_url);
    let res = match auth {
        Auth::QueryPassword(user) => req.json(&user),
        Auth::QueryToken(token) => req.json(&Token {
            token: token.to_string(),
        }),
        _ => unreachable!("login may not be called when using auth method `None`"),
    }
    .send()
    .await?;

    // Handle smarthome-errors which could occur during login
    match res.status() {
        StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
        status => Err(Error::Smarthome(status)),
    }
}
