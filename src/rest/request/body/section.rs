use serde::Serialize;

use crate::rest::models::Section;

#[derive(Serialize, Debug)]
pub struct CreateSectionBody {
    name: String,
    project_id: String,
    order: Option<isize>,
}

impl CreateSectionBody {
    pub fn new(name: &str, project_id: &str) -> CreateSectionBody {
        CreateSectionBody {
            name: name.to_string(),
            project_id: project_id.to_string(),
            order: None,
        }
    }

    pub fn set_name(&mut self, name: &str) -> &mut CreateSectionBody {
        self.name = name.to_string();
        self
    }

    pub fn set_project_id(&mut self, project_id: &str) -> &mut CreateSectionBody {
        self.project_id = project_id.to_string();
        self
    }

    pub fn set_order(&mut self, order: isize) -> &mut CreateSectionBody {
        self.order = Some(order);
        self
    }

    pub fn with_name(mut self, name: &str) -> CreateSectionBody {
        self.name = name.to_string();
        self
    }

    pub fn with_project_id(mut self, project_id: &str) -> CreateSectionBody {
        self.project_id = project_id.to_string();
        self
    }

    pub fn with_order(mut self, order: isize) -> CreateSectionBody {
        self.set_order(order);
        self
    }
}

impl From<&Section> for CreateSectionBody {
    fn from(section: &Section) -> CreateSectionBody {
        CreateSectionBody {
            name: section.name.clone(),
            project_id: section.project_id.clone(),
            order: Some(section.order),
        }
    }
}
