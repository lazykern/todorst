use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    pub file_name: String,

    pub file_type: String,

    pub file_url: String,

    pub resource_type: String,
}
