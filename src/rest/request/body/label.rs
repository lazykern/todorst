use serde::Serialize;

use crate::{rest::model::PersonalLabel, types::Color};

#[derive(Serialize, Debug)]
pub struct CratePersonalLabelBody {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_favorite: Option<bool>,
}

impl CratePersonalLabelBody {
    pub fn new(name: &str) -> CratePersonalLabelBody {
        CratePersonalLabelBody {
            name: name.to_string(),
            order: None,
            color: None,
            is_favorite: None,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_order(&mut self, order: isize) {
        self.order = Some(order);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn set_is_favorite(&mut self, is_favorite: bool) {
        self.is_favorite = Some(is_favorite);
    }

    pub fn with_name(mut self, name: &str) -> CratePersonalLabelBody {
        self.set_name(name);
        self
    }

    pub fn with_order(mut self, order: isize) -> CratePersonalLabelBody {
        self.set_order(order);
        self
    }

    pub fn with_color(mut self, color: Color) -> CratePersonalLabelBody {
        self.set_color(color);
        self
    }

    pub fn with_is_favorite(mut self, is_favorite: bool) -> CratePersonalLabelBody {
        self.set_is_favorite(is_favorite);
        self
    }
}

impl From<&PersonalLabel> for CratePersonalLabelBody {
    fn from(label: &PersonalLabel) -> Self {
        CratePersonalLabelBody {
            name: label.name.clone(),
            order: Some(label.order),
            color: Some(label.color.clone()),
            is_favorite: Some(label.is_favorite),
        }
    }
}
#[derive(Serialize, Debug)]
pub struct UpdatePersonalLabelBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_favorite: Option<bool>,
}

impl UpdatePersonalLabelBody {
    pub fn new() -> UpdatePersonalLabelBody {
        UpdatePersonalLabelBody {
            name: None,
            order: None,
            color: None,
            is_favorite: None,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn set_order(&mut self, order: isize) {
        self.order = Some(order);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn set_is_favorite(&mut self, is_favorite: bool) {
        self.is_favorite = Some(is_favorite);
    }

    pub fn with_name(mut self, name: &str) -> UpdatePersonalLabelBody {
        self.set_name(name);
        self
    }

    pub fn with_order(mut self, order: isize) -> UpdatePersonalLabelBody {
        self.set_order(order);
        self
    }

    pub fn with_color(mut self, color: Color) -> UpdatePersonalLabelBody {
        self.set_color(color);
        self
    }

    pub fn with_is_favorite(mut self, is_favorite: bool) -> UpdatePersonalLabelBody {
        self.set_is_favorite(is_favorite);
        self
    }
}

impl From<&PersonalLabel> for UpdatePersonalLabelBody {
    fn from(label: &PersonalLabel) -> Self {
        UpdatePersonalLabelBody {
            name: Some(label.name.clone()),
            order: Some(label.order),
            color: Some(label.color.clone()),
            is_favorite: Some(label.is_favorite),
        }
    }
}
