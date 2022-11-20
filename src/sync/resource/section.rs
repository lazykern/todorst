use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub id: String,

    pub name: String,

    pub project_id: String,

    pub section_order: isize,

    pub collapsed: bool,

    pub user_id: String,

    pub sync_id: String,

    pub is_deleted: bool,

    pub is_archived: bool,

    pub archived_at: Option<String>,

    pub added_at: String,
}
