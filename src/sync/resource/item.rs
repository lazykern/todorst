use serde::{Deserialize, Serialize};

use crate::types::Priority;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,

    pub user_id: String,

    pub project_id: String,

    pub content: String,

    pub description: String,

    pub priority: Priority,

    pub parent_id: Option<String>,

    pub child_order: isize,

    pub section_id: Option<String>,

    pub day_order: isize,

    pub collapsed: bool,

    pub labels: Vec<String>,

    pub added_by_uid: Option<String>,

    pub assigned_by_uid: Option<String>,

    pub responsible_uid: Option<String>,

    pub checked: bool,

    pub is_deleted: bool,

    pub sync_id: String,

    pub completed_at: Option<String>,

    pub added_at: String,
}
