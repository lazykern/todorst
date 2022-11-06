use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    #[serde(rename = "comment_count")]
    pub comment_count: i64,
    pub order: i64,
    pub color: String,
    #[serde(rename = "is_shared")]
    pub is_shared: bool,
    #[serde(rename = "is_favorite")]
    pub is_favorite: bool,
    #[serde(rename = "parent_id")]
    pub parent_id: String,
    #[serde(rename = "is_inbox_project")]
    pub is_inbox_project: bool,
    #[serde(rename = "is_team_inbox")]
    pub is_team_inbox: bool,
    #[serde(rename = "view_style")]
    pub view_style: String,
    pub url: String,
}

impl Project {
    pub fn from_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
