use serde::{Serialize, Serializer};
use serde_json::{json, Value};

use crate::rest::models::{Color, Project};

pub struct CreateProjectBody {
    json: Value,
}

impl CreateProjectBody {
    pub fn new(name: &str) -> CreateProjectBody {
        let json = json!({
            "name": name,
        });
        CreateProjectBody { json }
    }

    pub fn set_name(&mut self, name: &str) {
        self.json["name"] = json!(name);
    }

    pub fn set_parent_id(&mut self, parent_id: &str) {
        self.json["parent_id"] = json!(parent_id);
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

    pub fn set_view_style(&mut self, view_style: &str) {
        self.json["view_style"] = json!(view_style);
    }

    pub fn with_parent_id(mut self, parent_id: &str) -> CreateProjectBody {
        self.set_parent_id(parent_id);
        self
    }

    pub fn with_color(mut self, color: Color) -> CreateProjectBody {
        self.set_color(color);
        self
    }

    pub fn with_color_str(mut self, color: &str) -> CreateProjectBody {
        self.set_color_str(color);
        self
    }

    pub fn with_is_favorite(mut self, is_favorite: bool) -> CreateProjectBody {
        self.set_is_favorite(is_favorite);
        self
    }

    pub fn with_view_style(mut self, view_style: &str) -> CreateProjectBody {
        self.set_view_style(view_style);
        self
    }
}

impl From<&Project> for CreateProjectBody {
    fn from(project: &Project) -> CreateProjectBody {
        let json = json!({
            "name": project.name,
            "parent_id": project.parent_id,
            "color": project.color,
            "is_favorite": project.is_favorite,
            "view_style": project.view_style,
        });
        CreateProjectBody { json }
    }
}

impl Serialize for CreateProjectBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.json.serialize(serializer)
    }
}

pub struct UpdateProjectBody {
    json: Value,
}

impl UpdateProjectBody {
    pub fn new() -> UpdateProjectBody {
        let json = json!({});
        UpdateProjectBody { json }
    }

    pub fn set_name(&mut self, name: &str) {
        self.json["name"] = json!(name);
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

    pub fn set_view_style(&mut self, view_style: &str) {
        self.json["view_style"] = json!(view_style);
    }

    pub fn with_name(mut self, name: &str) -> UpdateProjectBody {
        self.json["name"] = json!(name);
        self
    }

    pub fn with_color(mut self, color: Color) -> UpdateProjectBody {
        self.json["color"] = json!(color);
        self
    }

    pub fn with_color_str(mut self, color: &str) -> UpdateProjectBody {
        self.json["color"] = json!(color);
        self
    }

    pub fn with_is_favorite(mut self, is_favorite: bool) -> UpdateProjectBody {
        self.json["is_favorite"] = json!(is_favorite);
        self
    }

    pub fn with_view_style(mut self, view_style: &str) -> UpdateProjectBody {
        self.json["view_style"] = json!(view_style);
        self
    }
}

impl From<&Project> for UpdateProjectBody {
    fn from(project: &Project) -> UpdateProjectBody {
        let json = json!({
            "name": project.name,
            "color": project.color,
            "is_favorite": project.is_favorite,
            "view_style": project.view_style,
        });
        UpdateProjectBody { json }
    }
}

impl Serialize for UpdateProjectBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.json.serialize(serializer)
    }
}
