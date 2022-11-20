use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Due {
    pub date: String,

    pub timezone: Option<String>,

    pub string: String,

    pub lang: Option<String>,

    pub is_recurring: bool,
}
