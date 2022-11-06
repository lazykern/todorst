use chrono::{DateTime, TimeZone};
use chrono_tz::{Tz, TZ_VARIANTS};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Due {
    pub date: String,
    #[serde(rename = "is_recurring")]
    pub is_recurring: bool,
    pub datetime: Option<String>,
    pub string: String,
    pub timezone: Option<String>,
}

impl Due {}
