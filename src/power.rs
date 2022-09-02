use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::errors::{Error, Result};
use crate::Client;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PowerRequest<'request> {
    switch: &'request str,
    power_on: bool,
}

#[derive(Deserialize, Debug)]
pub struct PowerDrawPoint {
    pub id: u64,
    pub time: u64,
    pub on: PowerDrawData,
    pub off: PowerDrawData,
}

#[derive(Deserialize, Debug)]
pub struct PowerDrawData {
    #[serde(rename = "switchCount")]
    pub switch_count: usize,
    pub watts: usize,
    pub percent: f64,
}

impl Client {
    /// Sets the power state of the given switch to the given value
    /// Still depends on user permissions and switch existence
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///
    ///     let res = client.set_power(
    ///             "test-switch",
    ///             false, /* Will turn off the test switch */
    ///     ).await.unwrap();
    /// }
    /// ```
    pub async fn set_power(&self, switch: &str, power_on: bool) -> Result<()> {
        let response = self
            .client
            .execute(self.build_request::<PowerRequest>(
                Method::POST,
                "/api/power/set",
                Some(PowerRequest { switch, power_on }),
            )?)
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(Error::Smarthome(status)),
        }
    }

    /// Returns power usage data from the server
    /// If `fetch_all` is set to `true`, all power measurements will be fetched from the server
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///
    ///     let res = client.power_usage(
    ///             false, /* Will only fetch data from the last 24 hours */
    ///     ).await.unwrap();
    /// }
    /// ```
    pub async fn power_usage(&self, fetch_all: bool) -> Result<Vec<PowerDrawPoint>> {
        let response = self
            .client
            .execute(self.build_request::<()>(
                Method::GET,
                if fetch_all {
                    "/api/power/usage/all"
                } else {
                    "/api/power/usage/day"
                },
                None,
            )?)
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<Vec<PowerDrawPoint>>().await?),
            status => Err(Error::Smarthome(status)),
        }
    }
}
