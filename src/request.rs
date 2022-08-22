use reqwest::{Request, Response};
use serde::Serialize;

use crate::errors::Result;
use crate::Auth::{QueryPassword, QueryToken, SessionPassword, SessionToken};
use crate::Client;

impl Client {
    /// Wrapper around the internal `build_request` function
    pub async fn get(&self, path: &str) -> Result<Response> {
        Ok(self
            .client
            .execute(self.build_request::<()>(reqwest::Method::GET, path, None)?)
            .await?)
    }

    /// Wrapper around `reqwest` which automatically handles authentication and body attachment
    pub fn build_request<T: Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<T>,
    ) -> Result<Request> {
        // Set the URL to use the desired path
        let mut url = self.smarthome_url.clone();
        url.set_path(path);
        // Create a request
        let mut request = self.client.request(method, url);
        // Depending on the authentication mode, choose a query-type
        match &self.auth {
            crate::Auth::None | SessionPassword(_) | SessionToken(_) => (),
            QueryPassword(user) => {
                request =
                    request.query(&[("username", &user.username), ("password", &user.password)])
            }
            QueryToken(token) => request = request.query(&[("token", token)]),
        };
        // Append a body if needed
        match body {
            Some(b) => Ok(request.json(&b).build()?),
            None => Ok(request.build()?),
        }
    }
}
