use reqwest::{Method, StatusCode};

use crate::{
    errors::{Error, Result},
    Client,
};

impl Client {
    /// Fetches an `export.json` file from the Smarthome server
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(foo", Auth::None).await.unwrap();
    ///
    ///     let res = client.export_config().await.unwrap();
    /// }
    /// ```
    pub async fn export_config(&self) -> Result<String> {
        let response = self
            .client
            .execute(self.build_request::<()>(
                Method::GET,
                "/api/system/config/export",
                None,
            )?)
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.text().await?),
            status => Err(Error::Smarthome(status)),
        }
    }
}
