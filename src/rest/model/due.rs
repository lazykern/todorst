use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Due {
    pub string: String,

    pub date: String,

    pub is_recurring: bool,

    pub datetime: Option<String>,

    pub timezone: Option<String>,
}
