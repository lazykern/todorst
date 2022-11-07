use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::due::Due;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct QuickAddResponse {
    pub added_by_uid: String,

    pub assigned_by_uid: String,

    pub labels: Vec<String>,

    pub sync_id: Value,

    pub added_at: String,

    pub parent_id: Value,

    pub content: String,

    pub description: String,

    pub is_deleted: bool,

    pub user_id: String,

    pub due: Option<Due>,

    pub id: String,

    pub priority: u8,

    pub child_order: isize,

    pub responsible_uid: String,

    pub project_id: String,

    pub collapsed: bool,

    pub checked: bool,
}
