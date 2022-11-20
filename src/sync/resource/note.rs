use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::file_attachment::FileAttachment;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemNote {
    pub id: String,

    pub posted_uid: String,

    pub item_id: String,

    pub content: String,

    pub file_attachment: Option<FileAttachment>,

    pub uids_to_notify: Option<Vec<String>>,

    pub is_deleted: bool,

    pub posted_at: String,

    pub reactions: Option<HashMap<String, Vec<String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNote {
    pub id: String,

    pub posted_uid: String,

    pub project_id: String,

    pub content: String,

    pub file_attachment: Option<FileAttachment>,

    pub uids_to_notify: Option<Vec<String>>,

    pub is_deleted: bool,

    pub posted_at: String,

    pub reactions: Option<HashMap<String, Vec<String>>>,
}
