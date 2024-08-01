use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};

use crate::errors::{Error, Result};
use crate::{Client, HomescriptExecError};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeviceRequest<'request> {
    device_id: &'request str,
    power: Option<DevicePowerRequest>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DevicePowerRequest {
    state: bool,
}

//
// DEVICE.
//

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedDeviceResponse {
    pub shallow: ShallowDeviceResponse,
    pub extractions: DeviceExtractions,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceExtractions {
    pub hms_errors: Vec<HomescriptExecError>,
    pub config: ConfigSpecWrapper,
    pub power_information: Option<DevicePowerInformation>,
    pub dimmables: Option<Vec<DeviceDimmable>>,
    pub sensors: Option<Vec<DeviceSensor>>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSpecWrapper {
    pub capabilities: Vec<DeviceCapability>,
    pub info: serde_json::Value,
}

#[derive(Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DeviceCapability {
    Base,
    Power,
    Dimmable,
    Sensor,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceType {
    Input,
    Output,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShallowDeviceResponse {
    #[serde(rename = "type")]
    pub type_: DeviceType,
    pub id: String,
    pub name: String,
    pub room_id: String,
    pub vendor_id: String,
    pub model_id: String,
    pub singleton_json: serde_json::Value,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DevicePowerInformation {
    pub state: bool,
    pub power_draw_watts: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDimmableRange {
    lower: f64,
    // Upper is always exclusive.
    upper: f64,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimmableRange {
    pub lower: f64,
    pub upper: f64,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDimmable {
    pub value: f64,
    pub label: String,
    pub range: DimmableRange,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSensor {
    pub label: String,
    pub value: serde_json::Value,
    pub hms_type: String,
    pub unit: String,
}

// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RichDevice {
//     pub id: String,
//     pub name: String,
//     pub room_id: String,
//     pub power_on: bool,
//     pub watts: u16,
// }

//
// END DEVICE.
//

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
            .execute(self.build_request::<DeviceRequest>(
                Method::POST,
                "/api/devices/action/power",
                Some(DeviceRequest {
                    device_id: switch,
                    power: Some(DevicePowerRequest { state: power_on }),
                }),
            )?)
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(Error::Smarthome(status)),
        }
    }

    /// Returns the personal switches of the current user
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///
    ///     let res = client.personal_switches().await.unwrap();
    /// }
    /// ```
    pub async fn personal_switches(&self) -> Result<Vec<HydratedDeviceResponse>> {
        let response = self
            .client
            .execute(self.build_request::<()>(
                Method::GET,
                "/api/devices/list/personal/rich",
                None,
            )?)
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json::<Vec<HydratedDeviceResponse>>().await?),
            status => Err(Error::Smarthome(status)),
        }
    }

    /// Returns all switches of the target system
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///
    ///     let res = client.all_switches().await.unwrap();
    /// }
    /// ```
    pub async fn all_switches(&self) -> Result<Vec<HydratedDeviceResponse>> {
        let response = self
            .client
            .execute(self.build_request::<()>(Method::GET, "/api/devices/list/all/rich", None)?)
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json::<Vec<HydratedDeviceResponse>>().await?),
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
    ///     // Will only fetch data from the last 24 hours
    ///     let res = client.power_usage(false).await.unwrap();
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
