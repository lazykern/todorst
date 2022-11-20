use serde::{Deserialize, Serialize};

use crate::types::Color;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalLabel {
    pub id: String,

    pub name: String,

    pub color: Color,

    pub item_order: isize,

    pub is_deleted: bool,

    pub is_favorite: bool,
}

pub type SharedLabel = String;
