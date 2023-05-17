use std::collections::HashMap;
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
    pub value: &'request str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptExecResponse {
    pub id: String,
    pub success: bool,
    pub exit_code: isize,
    pub output: String,
    pub file_contents: HashMap<String, String>,
    pub errors: Vec<HomescriptExecError>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptExecError {
    pub kind: String,
    pub span: HomescriptExecErrorSpan,
    pub message: String,
}

impl Display for HomescriptExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {}:{}\n  {}",
            self.kind, self.span.start.line, self.span.start.column, self.message,
        )
    }
}

impl HomescriptExecError {
    pub fn display(&self, code: &str) -> String {
        let lines = code.split('\n').collect::<Vec<&str>>();

        let line1 = if self.span.start.line > 1 {
            format!(
                "\n \x1b[90m{: >3} | \x1b[0m{}",
                self.span.start.line - 1,
                lines[self.span.start.line - 2]
            )
        } else {
            String::new()
        };
        let line2 = format!(
            " \x1b[90m{: >3} | \x1b[0m{}",
            self.span.start.line,
            lines[self.span.start.line - 1]
        );
        let line3 = if self.span.start.line < lines.len() {
            format!(
                "\n \x1b[90m{: >3} | \x1b[0m{}",
                self.span.start.line + 1,
                lines[self.span.start.line]
            )
        } else {
            String::new()
        };

        let (raw_marker, color) = match self.kind.as_ref() {
            "Info" => ("~", 6),
            "Warning" => ("~", 3),
            _ => ("^", 1),
        };

        let markers = if self.span.start.line == self.span.end.line {
            raw_marker.repeat(self.span.end.column - self.span.start.column + 1)
        } else {
            raw_marker.to_string()
        };

        let marker = format!("{}\x1b[1;3{}m{}\x1b[0m", " ".repeat(self.span.start.column + 6), color, markers);

        format!(
            "\x1b[1;3{}m{}\x1b[39m at {}:{}:{}\x1b[0m\n{}\n{}\n{}{}\n\n\x1b[1;3{}m{}\x1b[0m\n",
            color,
            self.kind,
            self.span.filename,
            self.span.start.line,
            self.span.start.column,
            line1,
            line2,
            marker,
            line3,
            color,
            self.message,
        )
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptExecErrorSpan {
    pub start: HomescriptExecErrorLocation,
    pub end: HomescriptExecErrorLocation,
    pub filename: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptExecErrorLocation {
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
