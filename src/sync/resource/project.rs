use crate::types::{Color, ViewStyle};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,

    pub name: String,

    pub color: Color,

    pub parent_id: Option<String>,

    pub child_order: isize,

    pub collapsed: bool,

    pub shared: bool,

    pub is_deleted: bool,

    pub is_archived: bool,

    pub is_favorite: bool,

    pub sync_id: Option<String>,

    #[serde(default)]
    pub inbox_project: Option<bool>,

    #[serde(default)]
    pub team_inbox: Option<bool>,

    view_style: ViewStyle,
}
