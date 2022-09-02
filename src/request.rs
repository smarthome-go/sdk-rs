use reqwest::Request;
use serde::Serialize;

use crate::errors::Result;
use crate::Auth;
use crate::Client;

impl Client {
    /// Wrapper around `reqwest` which automatically handles authentication and body attachment
    pub fn build_request<T: Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<T>,
    ) -> Result<Request> {
        // Create a request
        let mut request = self.client.request(method, self.smarthome_url.join(path)?);
        // Depending on the authentication mode, choose a query-type
        request = match &self.auth {
            Auth::None => request,
            Auth::QueryPassword(user) => {
                request.query(&[("username", &user.username), ("password", &user.password)])
            }
            Auth::QueryToken(token) => request.query(&[("token", token)]),
        };
        // Append a body if needed
        match body {
            Some(b) => Ok(request.json(&b).build()?),
            None => Ok(request.build()?),
        }
    }
}
