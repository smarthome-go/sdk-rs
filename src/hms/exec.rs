use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::{Error, Result};

#[derive(Serialize)]
pub struct ExecHomescriptbyIdRequest<'request> {
    pub id: &'request str,
    pub args: Vec<HomescriptArg<'request>>,
}

#[derive(Serialize)]
pub struct ExecHomescriptCodeRequest<'request> {
    pub code: &'request str,
    pub args: Vec<HomescriptArg<'request>>,
}

#[derive(Serialize)]
pub struct HomescriptArg<'request> {
    pub key: &'request str,
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
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///
    ///     let res = client.exec_homescript_code(
    ///             "print('Homescript is cool!')",
    ///             vec![], /* We dont need arguments for this example */
    ///             false, /* If set to true, the code would only be linted instead of executed */
    ///     ).await.unwrap();
    /// }
    /// ```
    pub async fn exec_homescript_code(
        &self,
        code: &str,
        args: Vec<HomescriptArg<'_>>,
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
            reqwest::StatusCode::OK | reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                Ok(result.json::<HomescriptExecResponse>().await?)
            }
            status => Err(Error::Smarthome(status)),
        }
    }

    /// Executes a Homescript (by its id) on the target server and returns the response
    /// The Homescript has to already exist on the target server
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///
    ///     let res = client.exec_homescript(
    ///             "test-script",
    ///             vec![], /* We dont need arguments for this example */
    ///             false, /* If set to true, the code would only be linted instead of executed */
    ///     ).await.unwrap();
    /// }
    /// ```
    pub async fn exec_homescript(
        &self,
        id: &str,
        args: Vec<HomescriptArg<'_>>,
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
            reqwest::StatusCode::OK | reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                Ok(result.json::<HomescriptExecResponse>().await?)
            }
            status => Err(Error::Smarthome(status)),
        }
    }
}
