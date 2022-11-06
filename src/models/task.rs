use crate::models::due::Due;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "assignee_id")]
    pub assignee_id: Option<String>,
    #[serde(rename = "assigner_id")]
    pub assigner_id: Option<String>,
    #[serde(rename = "comment_count")]
    pub comment_count: i64,
    #[serde(rename = "is_completed")]
    pub is_completed: bool,
    pub content: String,
    pub description: String,
    pub due: Option<Due>,
    pub id: String,
    pub labels: Vec<String>,
    pub order: i64,
    pub priority: i64,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "section_id")]
    pub section_id: Option<String>,
    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,
    pub url: String,
}

impl Task {
    pub fn from_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
