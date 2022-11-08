use super::Color;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,

    pub name: String,

    pub color: Color,

    pub parent_id: Option<String>,

    pub order: isize,

    pub comment_count: isize,

    pub is_shared: bool,

    pub is_favorite: bool,

    pub is_inbox_project: bool,

    pub is_team_inbox: bool,

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
