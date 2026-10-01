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
    // The server reports the type but rejects it as an unknown field in create / modify requests
    #[serde(rename = "type", skip_serializing)]
    pub type_: HomescriptType,
    pub workspace: String,
}

#[derive(Serialize)]
struct DeleteHomescriptRequest<'request> {
    id: &'request str,
}

#[derive(Serialize)]
struct ModifyHomescriptCodeRequest<'request> {
    id: &'request str,
    code: &'request str,
}

impl Client {
    /// Creates a new Homescript on the target server
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth, HomescriptData, HomescriptType};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None, true).await.unwrap();
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
    ///         is_widget: false,
    ///         type_: HomescriptType::Normal,
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
            _ => Err(Error::from_response(result).await),
        }
    }

    /// Modifies a Homescript's data
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth, HomescriptData, HomescriptType};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None, true).await.unwrap();
    ///
    ///     client.modify_homescript(&HomescriptData {
    ///         id: "".to_string(),
    ///         name: "".to_string(),
    ///         description: "".to_string(),
    ///         code: "".to_string(),
    ///         md_icon: "".to_string(),
    ///         workspace: "".to_string(),
    ///         scheduler_enabled: false,
    ///         quick_actions_enabled: false,
    ///         is_widget: false,
    ///         type_: HomescriptType::Normal,
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
            _ => Err(Error::from_response(result).await),
        }
    }

    /// Modifies only the code of a Homescript or driver script.
    /// Compile errors in the new code are reported via [`Error::SmarthomeResponse`].
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None, true).await.unwrap();
    ///
    ///     client.modify_homescript_code("foo-id", "fn main() {}").await.unwrap();
    /// }
    /// ```
    pub async fn modify_homescript_code(&self, id: &str, code: &str) -> Result<()> {
        let result = self
            .client
            .execute(self.build_request(
                reqwest::Method::PUT,
                "/api/homescript/modify/code",
                Some(ModifyHomescriptCodeRequest { id, code }),
            )?)
            .await?;
        match result.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => Err(Error::from_response(result).await),
        }
    }

    /// Deletes a Homescript from the target server
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth, HomescriptData, HomescriptType};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None, true).await.unwrap();
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
            _ => Err(Error::from_response(result).await),
        }
    }

    /// Returns a vec of the user's personal Homescripts
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None, true).await.unwrap();
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
            _ => Err(Error::from_response(result).await),
        }
    }
}
