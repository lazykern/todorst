use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileAttachment {
    pub file_type: String,

    pub file_name: String,

    pub file_size: u64,

    pub file_url: String,

    pub upload_state: String,
}
