use serde::{Deserialize, Serialize};

use crate::models::attachment::Attachment;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub content: String,
    pub id: String,
    #[serde(rename = "posted_at")]
    pub posted_at: String,
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[serde(rename = "task_id")]
    pub task_id: Option<String>,
    pub attachment: Option<Attachment>,
}
