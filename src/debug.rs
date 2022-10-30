use reqwest::Method;
use serde::Deserialize;

use crate::{errors::Result, Client, Error};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebugInfoData {
    #[serde(rename = "version")]
    pub server_version: String,
    pub go_version: String,
    pub cpu_cores: u8,
    pub goroutines: u16,
    pub memory_usage: u16,
    pub database_online: bool,
    pub database_stats: DatabaseStats,
    pub power_job_count: u16,
    #[serde(rename = "lastPowerJobErrorCount")]
    pub power_job_with_error_count: u16,
    pub power_jobs: Vec<PowerJob>,
    pub power_job_results: Vec<JobResult>,
    pub hardware_nodes_count: u16,
    pub hardware_nodes_online: u16,
    pub hardware_nodes_enabled: u16,
    pub hardware_nodes: Vec<HardwareNode>,
    pub homescript_job_count: u16,
    pub time: ServerTime,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStats {
    pub open_connections: i32,
    #[serde(rename = "InUse")]
    pub in_use: i32,
    #[serde(rename = "Idle")]
    pub idle: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerJob {
    pub id: i64,
    pub switch_name: String,
    pub power: bool,
}

#[derive(Deserialize, Debug)]
pub struct JobResult {
    pub id: i64,
    pub error: String,
}

#[derive(Deserialize, Debug)]
pub struct HardwareNode {
    pub name: String,
    pub online: bool,
    pub enabled: bool,
    pub url: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct ServerTime {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub unix: u64,
}

impl Client {
    pub async fn debug_info(&self) -> Result<DebugInfoData> {
        let response = self
            .client
            .execute(self.build_request::<Option<()>>(Method::GET, "/api/debug", None)?)
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<DebugInfoData>().await?),
            status => Err(Error::Smarthome(status)),
        }
    }
}
