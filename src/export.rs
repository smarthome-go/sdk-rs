use reqwest::{Method, StatusCode};
use serde::Serialize;

use crate::{
    errors::{Error, Result},
    Client,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRequest {
    pub include_profile_pictures: bool,
    pub include_cache_data: bool,
}

impl Client {
    /// Fetches an `export.json` file from the Smarthome server
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(foo", Auth::None).await.unwrap();
    ///
    ///     let res = client.export_config(ExportRequest {
    //          include_profile_pictures: false,
    //          include_cache_data: false,
    ///     }).await.unwrap();
    /// }
    /// ```
    pub async fn export_config(&self, request: &ExportRequest) -> Result<String> {
        let response = self
            .client
            .execute(self.build_request::<&ExportRequest>(
                Method::POST,
                "/api/system/config/export",
                Some(request),
            )?)
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.text().await?),
            status => Err(Error::Smarthome(status)),
        }
    }
}
