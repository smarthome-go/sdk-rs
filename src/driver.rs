use reqwest::{Method, StatusCode};
use serde::Deserialize;

use crate::errors::{Error, Result};
use crate::Client;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RichDriverResponse {
    pub driver: DriverData,
    /// Extracted driver / device configuration specs.
    pub info: serde_json::Value,
    /// The stored value(s) of the driver singleton.
    /// This is `null` if the driver has not been configured yet.
    pub configuration: serde_json::Value,
    pub is_valid: bool,
    pub validation_errors: Vec<serde_json::Value>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DriverData {
    pub vendor_id: String,
    pub model_id: String,
    pub name: String,
    pub version: String,
    pub homescript_code: String,
    pub singleton_json: Option<String>,
    pub dirty: bool,
}

impl Client {
    /// Lists all device drivers of the target system, including their
    /// singleton configuration.
    /// Requires the system-config permission.
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///
    ///     let res = client.list_drivers().await.unwrap();
    /// }
    /// ```
    pub async fn list_drivers(&self) -> Result<Vec<RichDriverResponse>> {
        let response = self
            .client
            .execute(self.build_request::<()>(
                Method::GET,
                "/api/system/hardware/driver/list",
                None,
            )?)
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json::<Vec<RichDriverResponse>>().await?),
            status => Err(Error::Smarthome(status)),
        }
    }
}
