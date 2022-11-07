use crate::models::due::Due;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub creator_id: String,

    pub created_at: String,

    pub assignee_id: Option<String>,

    pub assigner_id: Option<String>,

    pub comment_count: i64,

    pub is_completed: bool,

    pub content: String,

    pub description: String,

    pub due: Option<Due>,

    pub id: String,

    pub labels: Vec<String>,

    pub order: i64,

    pub priority: i64,

    pub project_id: String,

    pub section_id: Option<String>,

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
