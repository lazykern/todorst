use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "file_name")]
    pub file_name: String,
    #[serde(rename = "file_type")]
    pub file_type: String,
    #[serde(rename = "file_url")]
    pub file_url: String,
    #[serde(rename = "resource_type")]
    pub resource_type: String,
}
