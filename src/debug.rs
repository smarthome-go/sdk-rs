use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{errors::Result, Client, Error};

#[derive(Deserialize, Serialize, Debug)]
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
    pub homescript_job_count: u16,
    pub time: ServerTime,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStats {
    pub open_connections: i32,
    #[serde(rename = "InUse")]
    pub in_use: i32,
    #[serde(rename = "Idle")]
    pub idle: i32,
}

#[derive(Deserialize, Serialize, Debug)]
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
            _ => Err(Error::from_response(response).await),
        }
    }
}
