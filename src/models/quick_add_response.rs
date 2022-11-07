use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::due::Due;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickAddResponse {
    #[serde(rename = "added_by_uid")]
    pub added_by_uid: String,
    #[serde(rename = "assigned_by_uid")]
    pub assigned_by_uid: String,
    pub labels: Vec<String>,
    #[serde(rename = "sync_id")]
    pub sync_id: Value,
    #[serde(rename = "added_at")]
    pub added_at: String,
    #[serde(rename = "parent_id")]
    pub parent_id: Value,
    pub content: String,
    pub description: String,
    #[serde(rename = "is_deleted")]
    pub is_deleted: bool,
    #[serde(rename = "user_id")]
    pub user_id: String,
    pub due: Option<Due>,
    pub id: String,
    pub priority: i64,
    #[serde(rename = "child_order")]
    pub child_order: i64,
    #[serde(rename = "responsible_uid")]
    pub responsible_uid: String,
    #[serde(rename = "project_id")]
    pub project_id: String,
    pub collapsed: bool,
    pub checked: bool,
}
