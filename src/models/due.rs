use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Due {
    pub date: String,
    pub is_recurring: bool,
    pub string: String,

    pub datetime: Option<String>,
    pub timezone: Option<String>,
}

impl Due {
    pub fn from_str(json: &str) -> Result<Option<Self>, serde_json::Error> {
        let value: Value = serde_json::from_str(json)?;
        if value.is_null() {
            return Ok(None);
        }
        let due = serde_json::from_value(value)?;
        Ok(Some(due))
    }

    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
