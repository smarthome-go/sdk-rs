use std::fmt::Display;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::errors::{Error, Result};
use crate::client::Client;

#[derive(Serialize)]
pub struct ExecHomescriptbyIdRequest {
    pub id: String,
    pub args: Vec<HomescriptArg>,
}

#[derive(Serialize)]
pub struct ExecHomescriptCodeRequest {
    pub code: String,
    pub args: Vec<HomescriptArg>,
}

#[derive(Serialize)]
pub struct HomescriptArg {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptExecResponse {
    pub id: String,
    pub success: bool,
    pub exit_code: isize,
    pub message: String,
    pub output: String,
    #[serde(rename = "error")]
    pub errors: Vec<HomescriptExecError>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptExecError {
    pub error_type: String,
    pub location: HomescriptExecErrorLocation,
    pub message: String,
}

impl Display for HomescriptExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {}:{}:{}\n  {}",
            self.error_type,
            self.location.filename,
            self.location.line,
            self.location.column,
            self.message,
        )
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptExecErrorLocation {
    pub filename: String,
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

impl Client {
    /// Executes Homescript code on the target server and returns the response
    pub async fn exec_homescript_code(
        &self,
        code: String,
        args: Vec<HomescriptArg>,
        lint: bool,
    ) -> Result<HomescriptExecResponse> {
        let result = self
            .client
            .execute(self.build_request::<ExecHomescriptCodeRequest>(
                reqwest::Method::POST,
                if lint {
                    "/api/homescript/lint/live"
                } else {
                    "/api/homescript/run/live"
                },
                Some(ExecHomescriptCodeRequest { code, args }),
            )?)
            .await?;
        match result.status() {
            StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR => {
                Ok(result.json::<HomescriptExecResponse>().await?)
            }
            status => Err(Error::Smarthome(status)),
        }
    }

    /// Executes a Homescript by-id on the target server
    pub async fn exec_homescript(
        &self,
        id: String,
        args: Vec<HomescriptArg>,
        lint: bool,
    ) -> Result<HomescriptExecResponse> {
        let result = self
            .client
            .execute(self.build_request::<ExecHomescriptbyIdRequest>(
                reqwest::Method::POST,
                if lint {
                    "/api/homescript/lint"
                } else {
                    "/api/homescript/run"
                },
                Some(ExecHomescriptbyIdRequest { id, args }),
            )?)
            .await?;
        match result.status() {
            StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR => {
                Ok(result.json::<HomescriptExecResponse>().await?)
            }
            status => Err(Error::Smarthome(status)),
        }
    }
}
