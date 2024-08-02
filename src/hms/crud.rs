use serde::{Deserialize, Serialize};

use crate::{Client, Error, Result};

#[derive(Deserialize, Serialize, Debug)]
pub struct Homescript {
    pub owner: String,
    pub data: HomescriptData,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HomescriptType {
    Normal,
    Driver,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HomescriptData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quick_actions_enabled: bool,
    pub scheduler_enabled: bool,
    pub is_widget: bool,
    pub code: String,
    pub md_icon: String,
    #[serde(rename = "type")]
    pub type_: HomescriptType,
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
    ///         is_widged: false,
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

    /// Modifies a Homescript's data
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
    ///         is_widged: false,
    ///     }).await.unwrap();
    /// }
    /// ```
    pub async fn modify_homescript(&self, new_data: &HomescriptData) -> Result<()> {
        let result = self
            .client
            .execute(self.build_request::<&HomescriptData>(
                reqwest::Method::PUT,
                "/api/homescript/modify",
                Some(new_data),
            )?)
            .await?;
        match result.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(Error::Smarthome(status)),
        }
    }

    /// Deletes a Homescript from the target server
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

    /// Returns a vec of the user's personal Homescripts
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///
    ///     client.list_personal_homescripts().await.unwrap();
    /// }
    /// ```
    pub async fn list_personal_homescripts(&self) -> Result<Vec<Homescript>> {
        let result = self
            .client
            .execute(self.build_request::<()>(
                reqwest::Method::GET,
                "/api/homescript/list/personal",
                None,
            )?)
            .await?;
        match result.status() {
            reqwest::StatusCode::OK => Ok(result.json::<Vec<Homescript>>().await?),
            status => Err(Error::Smarthome(status)),
        }
    }
}
