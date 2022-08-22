use serde::Deserialize;

#[derive(Deserialize)]
pub struct Homescript {
    pub owner: String,
    pub data: HomescriptData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quick_actions_enabled: bool,
    pub scheduler_enabled: bool,
    pub code: String,
    pub md_icon: String,
}

pub mod exec {
    use serde::{Deserialize, Serialize};

    use crate::errors::{Error, Result};
    use crate::Client;

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

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct HomescriptExecErrorLocation {
        pub filename: String,
        pub line: usize,
        pub column: usize,
        pub index: usize,
    }

    impl Client {
        pub async fn run_homescript_code(
            &self,
            code: String,
            args: Vec<HomescriptArg>,
        ) -> Result<HomescriptExecResponse> {
            let result = self
                .client
                .execute(self.build_request::<ExecHomescriptCodeRequest>(
                    reqwest::Method::POST,
                    "/api/homescript/run/live",
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
    }
}
