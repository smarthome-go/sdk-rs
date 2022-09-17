use reqwest::{Method, StatusCode, Url};
use serde::Deserialize;

use crate::{
    auth::Token,
    errors::{Error, Result},
    version,
    version::VersionResponse,
    Auth, HTTP_USER_AGENT,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TokenResponse {
    username: String,
    _token_label: String,
}

/// The client represents an object-oriented approach to interact with the server
/// Any operation is implemented as a function on this struct
/// It is created using the associative new function
/// ```rust
/// // The SDK requires an aync runtime (Async excluded from this example)
/// use smarthome_sdk_rs::{Client, Auth};
/// ```
pub struct Client {
    pub client: reqwest::Client,
    pub auth: Auth,
    pub smarthome_url: Url,
    pub smarthome_version: VersionResponse,
    pub username: Option<String>,
}

impl Client {
    /// Creates a new client and validates the server's compatibility
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(
    ///         "https://smarthome.box",
    ///         Auth::None,
    ///     ).await.unwrap();
    /// }
    /// ```
    pub async fn new(raw_url: &str, auth: Auth) -> Result<Self> {
        // Parse the source url
        let smarthome_url = Url::parse(raw_url)?;
        // Default reqwest client with a pre-set user agent is created
        let client = reqwest::Client::builder()
            .user_agent(HTTP_USER_AGENT)
            .build()?;
        // Fetches the current version from the Smarthome server
        let mut version_url = smarthome_url.clone();
        version_url.set_path("/api/version");
        let res = reqwest::get(version_url).await?;
        // Handle errors which could occur during fetching
        let version = match res.status() {
            StatusCode::OK => res.json::<VersionResponse>().await?,
            code => return Err(Error::Smarthome(code)),
        };
        // Check if the SDK's version constraint is fulfilled by the server
        let mut client = match version::is_server_compatible(&version.smarthome_version) {
            Ok(true) => Self {
                client,
                auth,
                smarthome_url,
                smarthome_version: version,
                username: None,
            },
            Ok(false) => return Err(Error::IncompatibleVersion(version.smarthome_version)),
            Err(err) => return Err(err),
        };
        // Attempt to login using the credentials, makes sure that the client is operational
        // Also obtains the client's username in case token authentication is used
        match &client.auth {
            Auth::None => Ok(client),
            auth => {
                client.username = Some(login_with_credentials(&client.smarthome_url, auth).await?);
                Ok(client)
            }
        }
    }
}

/// Validates the client's credentials and returns a username
async fn login_with_credentials(base_url: &Url, auth: &Auth) -> Result<String> {
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
        StatusCode::OK | StatusCode::NO_CONTENT => match auth {
            Auth::QueryPassword(user) => Ok(user.username.clone()),
            Auth::QueryToken(_) => Ok(res.json::<TokenResponse>().await?.username),
            _ => unreachable!("This function may not be called with no authentication mode"),
        },
        status => Err(Error::Smarthome(status)),
    }
}
