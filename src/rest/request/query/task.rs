use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetTasksQuery {
    pub project_id: Option<String>,
    pub section_id: Option<String>,
    pub label: Option<String>,
    pub filter: Option<String>,
    pub ids: Option<Vec<isize>>,
}

impl GetTasksQuery {
    pub fn new() -> Self {
        Self {
            project_id: None,
            section_id: None,
            label: None,
            filter: None,
            ids: None,
        }
    }

    pub fn with_project_id(mut self, project_id: &str) -> Self {
        self.project_id = Some(project_id.to_string());
        self
    }

    pub fn with_section_id(mut self, section_id: &str) -> Self {
        self.section_id = Some(section_id.to_string());
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_filter(mut self, filter: String) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn with_ids(mut self, ids: Vec<isize>) -> Self {
        self.ids = Some(ids);
        self
    }

    pub fn set_project_id(&mut self, project_id: String) {
        self.project_id = Some(project_id);
    }

    pub fn set_section_id(&mut self, section_id: String) {
        self.section_id = Some(section_id);
    }

    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }

    pub fn set_filter(&mut self, filter: String) {
        self.filter = Some(filter);
    }

    pub fn set_ids(&mut self, ids: Vec<isize>) {
        self.ids = Some(ids);
    }
}
