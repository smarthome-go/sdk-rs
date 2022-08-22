use serde::{self, Deserialize, Serialize};

pub struct Homescript {
    pub owner: String,
    pub data: HomescriptData,
}

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
