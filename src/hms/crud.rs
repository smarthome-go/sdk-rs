use serde::{Deserialize, Serialize};

use crate::{Client, Error, Result};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quick_actions_enabled: bool,
    pub scheduler_enabled: bool,
    pub code: String,
    pub md_icon: String,
    pub workspace: String,
}

#[derive(Serialize)]
struct DeleteHomescriptRequest<'request> {
    id: &'request str,
}

impl Client {
    /// Creates a new Homescript on the target server
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth, HomescriptData};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///
    ///     client.create_homescript(&HomescriptData {
    ///         id: "".to_string(),
    ///         name: "".to_string(),
    ///         description: "".to_string(),
    ///         code: "".to_string(),
    ///         md_icon: "".to_string(),
    ///         workspace: "".to_string(),
    ///         scheduler_enabled: false,
    ///         quick_actions_enabled: false,
    ///     }).await.unwrap();
    /// }
    /// ```
    pub async fn create_homescript(&self, data: &HomescriptData) -> Result<()> {
        let result = self
            .client
            .execute(self.build_request::<&HomescriptData>(
                reqwest::Method::POST,
                "/api/homescript/add",
                Some(data),
            )?)
            .await?;
        match result.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(Error::Smarthome(status)),
        }
    }

    /// Delets a Homescript from the target server
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth, HomescriptData};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///
    ///     client.delete_homescript("foo-id").await.unwrap();
    /// }
    /// ```
    pub async fn delete_homescript(&self, id: &str) -> Result<()> {
        let result = self
            .client
            .execute(self.build_request::<DeleteHomescriptRequest>(
                reqwest::Method::DELETE,
                "/api/homescript/delete",
                Some(DeleteHomescriptRequest { id }),
            )?)
            .await?;
        match result.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(Error::Smarthome(status)),
        }
    }
}
