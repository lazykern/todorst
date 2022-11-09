use serde::Serialize;
use serde_json::{json, Value};

use crate::rest::models::Section;

pub struct CreateSectionBody {
    json: Value,
}

impl CreateSectionBody {
    pub fn new(name: &str, project_id: &str) -> CreateSectionBody {
        CreateSectionBody {
            json: json!({
                "name": name,
                "project_id": project_id,
            }),
        }
    }

    pub fn set_name(&mut self, name: &str) -> &mut CreateSectionBody {
        self.json["name"] = json!(name);
        self
    }

    pub fn set_project_id(&mut self, project_id: &str) -> &mut CreateSectionBody {
        self.json["project_id"] = json!(project_id);
        self
    }

    pub fn set_order(&mut self, order: u32) -> &mut CreateSectionBody {
        self.json["order"] = json!(order);
        self
    }

    pub fn with_order(mut self, order: u32) -> CreateSectionBody {
        self.json["order"] = json!(order);
        self
    }
}

impl From<&Section> for CreateSectionBody {
    fn from(section: &Section) -> CreateSectionBody {
        CreateSectionBody {
            json: json!({
                "name": section.name,
                "project_id": section.project_id,
                "order": section.order,
            }),
        }
    }
}

impl Serialize for CreateSectionBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.json.serialize(serializer)
    }
}

pub struct UpdateSectionBody {
    json: Value,
}

impl UpdateSectionBody {
    pub fn new(name: &str) -> UpdateSectionBody {
        UpdateSectionBody {
            json: json!({
                "name": name,
            }),
        }
    }

    pub fn set_name(&mut self, name: &str) -> &mut UpdateSectionBody {
        self.json["name"] = json!(name);
        self
    }
}

impl From<&Section> for UpdateSectionBody {
    fn from(section: &Section) -> UpdateSectionBody {
        UpdateSectionBody {
            json: json!({
                "name": section.name,
            }),
        }
    }
}

impl Serialize for UpdateSectionBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.json.serialize(serializer)
    }
}
