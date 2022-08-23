use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::errors::{Error, Result};
use crate::Client;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PowerRequest {
    switch: String,
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
    pub async fn set_power(&self, switch: String, power_on: bool) -> Result<()> {
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
