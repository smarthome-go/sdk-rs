use crate::{errors::Result, Client, Error};
use reqwest::Method;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Room {
    pub data: RoomData,
    pub switches: Vec<Switch>,
    pub cameras: Vec<Camera>,
}

#[derive(Deserialize, Debug)]
pub struct RoomData {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Switch {
    pub id: String,
    pub name: String,
    pub room_id: String,
    pub power_on: bool,
    pub watts: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub id: String,
    pub name: String,
    pub url: String,
    pub room_id: String,
}

impl Client {
    /// Returns a list containing the personal rooms of the current user
    /// ```rust no_run
    /// use smarthome_sdk_rs::{Client, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("foo", Auth::None).await.unwrap();
    ///     let res = client.personal_rooms().await.unwrap();
    /// }
    /// ```
    pub async fn personal_rooms(&self) -> Result<Vec<Room>> {
        let response = self
            .client
            .execute(self.build_request::<()>(Method::GET, "/api/room/list/personal", None)?)
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<Vec<Room>>().await?),
            status => Err(Error::Smarthome(status)),
        }
    }
}
