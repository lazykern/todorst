use serde::{Deserialize, Serialize};

use super::Color;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub id: String,

    pub name: String,

    pub color: Color,

    pub order: isize,

    pub is_favorite: bool,
}
