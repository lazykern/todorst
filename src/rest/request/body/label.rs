use serde::Serialize;
use serde_json::{json, Value};

use crate::rest::models::{Color, Label};

pub struct CratePersonalLabelBody {
    json: Value,
}

impl CratePersonalLabelBody {
    pub fn new(name: &str) -> CratePersonalLabelBody {
        let json = json!({
            "name": name,
        });
        CratePersonalLabelBody { json }
    }

    pub fn set_name(&mut self, name: &str) {
        self.json["name"] = json!(name);
    }

    pub fn set_order(&mut self, order: isize) {
        self.json["order"] = json!(order);
    }

    pub fn set_color(&mut self, color: Color) {
        self.json["color"] = json!(color);
    }

    pub fn set_color_str(&mut self, color: &str) {
        self.json["color"] = json!(color);
    }

    pub fn set_is_favorite(&mut self, is_favorite: bool) {
        self.json["is_favorite"] = json!(is_favorite);
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

    pub fn with_color_str(mut self, color: &str) -> CratePersonalLabelBody {
        self.set_color_str(color);
        self
    }

    pub fn with_is_favorite(mut self, is_favorite: bool) -> CratePersonalLabelBody {
        self.set_is_favorite(is_favorite);
        self
    }
}

impl From<&Label> for CratePersonalLabelBody {
    fn from(label: &Label) -> Self {
        let json = json!({
            "name": label.name,
            "color": label.color,
            "order": label.order,
            "is_favorite": label.is_favorite,
        });

        CratePersonalLabelBody { json }
    }
}

impl Serialize for CratePersonalLabelBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.json.serialize(serializer)
    }
}

pub type UpdatePersonalLabelBody = CratePersonalLabelBody;
