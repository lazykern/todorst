use serde::{Deserialize, Serialize};

use crate::models::attachment::Attachment;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub content: String,
    pub id: String,
    pub posted_at: String,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
    pub attachment: Option<Attachment>,
}
