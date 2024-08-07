use std::collections::HashMap;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::{Error, Result};

pub enum HmsRunMode<'request> {
    Execute,
    Lint {
        module_name: &'request str,
        is_driver: bool,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecHomescriptbyIdRequest<'request> {
    pub id: &'request str,
    pub args: Vec<HomescriptArg<'request>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LintHomescriptCodeRequest<'request> {
    pub code: &'request str,
    pub args: Vec<HomescriptArg<'request>>,
    pub module_name: &'request str,
    pub is_driver: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunHomescriptCodeRequest<'request> {
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
    pub success: bool,
    pub output: String,
    pub file_contents: HashMap<String, String>,
    pub errors: Vec<HomescriptExecError>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptExecError {
    pub syntax_error: Option<SyntaxError>,
    pub diagnostic_error: Option<DiagnosticError>,
    pub runtime_error: Option<RuntimeInterrupt>,
    pub span: HomescriptExecErrorSpan,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxError {
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticError {
    pub kind: u8,
    pub message: String,
    pub notes: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInterrupt {
    pub kind: String,
    pub message: String,
}

impl Display for HomescriptExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(syntax) = &self.syntax_error {
            return write!(
                f,
                "SyntaxError at {}:{}\n  {}",
                self.span.start.line, self.span.start.column, syntax.message,
            );
        }

        if let Some(diagnostic) = &self.diagnostic_error {
            let level = match diagnostic.kind {
                0 => "Hint",
                1 => "Info",
                2 => "Warning",
                3 => "Error",
                other => unreachable!("Unsupported level {other}"),
            };

            return write!(
                f,
                "{} at {}:{}\n  {}",
                level, self.span.start.line, self.span.start.column, diagnostic.message,
            );
        }

        if let Some(runtime) = &self.runtime_error {
            return write!(
                f,
                "{} at {}:{}\n  {}",
                runtime.kind, self.span.start.line, self.span.start.column, runtime.message,
            );
        }

        unreachable!("The structure of the exec response changed")
    }
}

impl HomescriptExecError {
    pub fn display(&self, code: &str) -> String {
        match (
            &self.syntax_error,
            &self.diagnostic_error,
            &self.runtime_error,
        ) {
            (Some(syntax), None, None) => {
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

                let markers = if self.span.start.line == self.span.end.line {
                    "^".repeat(self.span.end.column - self.span.start.column + 1)
                } else {
                    "^".to_string()
                };

                let marker = format!(
                    "{}\x1b[1;3{}m{}\x1b[0m",
                    " ".repeat(self.span.start.column + 6),
                    1,
                    markers
                );

                format!(
            "\x1b[1;3{}m{}\x1b[39m at {}:{}:{}\x1b[0m\n{}\n{}\n{}{}\n\n\x1b[1;3{}m{}\x1b[0m\n",
            1,
            "Error",
            self.span.filename,
            self.span.start.line,
            self.span.start.column,
            line1,
            line2,
            marker,
            line3,
            1,
            syntax.message,
        )
            }
            (None, Some(diagnostic), None) => {
                // take special action if there is no useful span / the source code is empty
                if self.span.start.line == 0
                    && self.span.start.column == 0
                    && self.span.end.line == 0
                    && self.span.end.column == 0
                {
                    let (kind, color) = match diagnostic.kind {
                        0 => ("Hint", 35),
                        1 => ("Info", 36),
                        2 => ("Warning", 33),
                        3 => ("Error", 31),
                        _ => unreachable!("Illegal configuration"),
                    };
                    return format!(
                        "\x1b[1;{color}m{kind}\x1b[1;0m\x1b[1;39m in {}\x1b[0m\n{}",
                        self.span.filename, diagnostic.message,
                    );
                }

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

                let (kind, raw_marker, color) = match diagnostic.kind {
                    0 => ("Hint", "~", 5),
                    1 => ("Info", "~", 6),
                    2 => ("Warning", "~", 3),
                    3 => ("Error", "^", 1),
                    other => unreachable!(
                        "A new level {other} was introduced without updating this code"
                    ),
                };

                let markers = if self.span.start.line == self.span.end.line {
                    raw_marker.repeat(self.span.end.column - self.span.start.column + 1)
                } else {
                    raw_marker.to_string()
                };

                let marker = format!(
                    "{}\x1b[1;3{}m{}\x1b[0m",
                    " ".repeat(self.span.start.column + 6),
                    color,
                    markers
                );

                format!(
            "\x1b[1;3{}m{}\x1b[39m at {}:{}:{}\x1b[0m\n{}\n{}\n{}{}\n\n\x1b[1;3{}m{}\x1b[0m\n",
            color,
            kind,
            self.span.filename,
            self.span.start.line,
            self.span.start.column,
            line1,
            line2,
            marker,
            line3,
            color,
            diagnostic.message,
        )
            }
            (None, None, Some(runtime)) => {
                // take special action if there is no useful span / the source code is empty
                if self.span.start.line == 0
                    && self.span.start.column == 0
                    && self.span.end.line == 0
                    && self.span.end.column == 0
                {
                    return format!(
                        "{}{}\x1b[1;39m in {}\x1b[0m\n{}",
                        31, runtime.kind, self.span.filename, runtime.message,
                    );
                }

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

                let markers = if self.span.start.line == self.span.end.line {
                    "^".repeat(self.span.end.column - self.span.start.column + 1)
                } else {
                    "^".to_string()
                };

                let marker = format!(
                    "{}\x1b[1;3{}m{}\x1b[0m",
                    " ".repeat(self.span.start.column + 6),
                    1,
                    markers
                );

                format!(
            "\x1b[1;3{}mRuntimeError\x1b[39m at {}:{}:{}\x1b[0m\n{}\n{}\n{}{}\n\n\x1b[1;3{}m{}\x1b[0m\n",
            1,
            self.span.filename,
            self.span.start.line,
            self.span.start.column,
            line1,
            line2,
            marker,
            line3,
            1,
            runtime.message,
        )
            }
            (_, _, _) => unreachable!("Illegal state"),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptExecErrorSpan {
    pub start: HomescriptExecErrorLocation,
    pub end: HomescriptExecErrorLocation,
    pub filename: String,
}

#[derive(Deserialize, Debug, Clone)]
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
    ///             "println('Homescript is cool!')",
    ///             vec![], /* We dont need arguments for this example */
    ///             false, /* If set to true, the code would only be linted instead of executed */
    ///     ).await.unwrap();
    /// }
    /// ```
    pub async fn exec_homescript_code(
        &self,
        code: &str,
        args: Vec<HomescriptArg<'_>>,
        run_mode: HmsRunMode<'_>,
    ) -> Result<HomescriptExecResponse> {
        let req = match run_mode {
            HmsRunMode::Execute => self.build_request::<RunHomescriptCodeRequest>(
                reqwest::Method::POST,
                "/api/homescript/run/live",
                Some(RunHomescriptCodeRequest { code, args }),
            )?,
            HmsRunMode::Lint {
                module_name,
                is_driver,
            } => self.build_request::<LintHomescriptCodeRequest>(
                reqwest::Method::POST,
                "/api/homescript/lint/live",
                Some(LintHomescriptCodeRequest {
                    code,
                    args,
                    module_name,
                    is_driver,
                }),
            )?,
        };

        let result = self.client.execute(req).await?;
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
        let url = match lint {
            false => "/api/homescript/run",
            true => "/api/homescript/lint",
        };
        let result = self
            .client
            .execute(self.build_request::<ExecHomescriptbyIdRequest>(
                reqwest::Method::POST,
                url,
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
