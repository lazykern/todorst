use serde::Serialize;

// use crate::rest::models::Task;

#[derive(Serialize, Debug)]
pub struct CreateTaskBody {
    content: String,
    description: Option<String>,
    project_id: Option<String>,
    section_id: Option<String>,
    parent_id: Option<String>,
    order: Option<isize>,
    labels: Option<Vec<String>>,
    priority: Option<isize>,
    due_string: Option<String>,
    due_date: Option<String>,
    due_datetime: Option<String>,
    due_lang: Option<String>,
    assignee_id: Option<String>,
}

impl CreateTaskBody {
    pub fn new(content: &str) -> CreateTaskBody {
        CreateTaskBody {
            content: content.to_string(),
            description: None,
            project_id: None,
            section_id: None,
            parent_id: None,
            order: None,
            labels: None,
            priority: None,
            due_string: None,
            due_date: None,
            due_datetime: None,
            due_lang: None,
            assignee_id: None,
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    pub fn set_project_id(&mut self, project_id: &str) {
        self.project_id = Some(project_id.to_string());
    }

    pub fn set_section_id(&mut self, section_id: &str) {
        self.section_id = Some(section_id.to_string());
    }

    pub fn set_parent_id(&mut self, parent_id: &str) {
        self.parent_id = Some(parent_id.to_string());
    }

    pub fn set_order(&mut self, order: isize) {
        self.order = Some(order);
    }

    pub fn set_labels(&mut self, labels: Vec<&str>) {
        self.labels = Some(labels.iter().map(|s| s.to_string()).collect());
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = Some(priority as isize);
    }

    pub fn set_due_string(&mut self, due_string: &str) {
        self.due_string = Some(due_string.to_string());

        if self.due_date.is_some() {
            self.due_date = None;
        }
        if self.due_datetime.is_some() {
            self.due_datetime = None;
        }
    }

    pub fn set_due_date(&mut self, due_date: &str) {
        self.due_date = Some(due_date.to_string());

        if self.due_string.is_some() {
            self.due_string = None;
        }
        if self.due_datetime.is_some() {
            self.due_datetime = None;
        }
    }

    pub fn set_due_datetime(&mut self, due_datetime: &str) {
        self.due_datetime = Some(due_datetime.to_string());

        if self.due_string.is_some() {
            self.due_string = None;
        }
        if self.due_date.is_some() {
            self.due_date = None;
        }
    }

    pub fn set_due_lang(&mut self, due_lang: &str) {
        self.due_lang = Some(due_lang.to_string());
    }

    pub fn set_assignee_id(&mut self, assignee_id: &str) {
        self.assignee_id = Some(assignee_id.to_string());
    }

    pub fn with_description(mut self, description: &str) -> CreateTaskBody {
        self.set_description(description);
        self
    }

    pub fn with_project_id(mut self, project_id: &str) -> CreateTaskBody {
        self.set_project_id(project_id);
        self
    }

    pub fn with_section_id(mut self, section_id: &str) -> CreateTaskBody {
        self.set_section_id(section_id);
        self
    }

    pub fn with_parent_id(mut self, parent_id: &str) -> CreateTaskBody {
        self.set_parent_id(parent_id);
        self
    }

    pub fn with_order(mut self, order: isize) -> CreateTaskBody {
        self.set_order(order);
        self
    }

    pub fn with_labels(mut self, labels: Vec<&str>) -> CreateTaskBody {
        self.set_labels(labels);
        self
    }

    pub fn with_priority(mut self, priority: u8) -> CreateTaskBody {
        self.set_priority(priority);
        self
    }

    pub fn with_due_string(mut self, due_string: &str) -> CreateTaskBody {
        self.set_due_string(due_string);
        self
    }

    pub fn with_due_date(mut self, due_date: &str) -> CreateTaskBody {
        self.set_due_date(due_date);
        self
    }

    pub fn with_due_datetime(mut self, due_datetime: &str) -> CreateTaskBody {
        self.set_due_datetime(due_datetime);
        self
    }

    pub fn with_due_lang(mut self, due_lang: &str) -> CreateTaskBody {
        self.set_due_lang(due_lang);
        self
    }

    pub fn with_assignee_id(mut self, assignee_id: &str) -> CreateTaskBody {
        self.set_assignee_id(assignee_id);
        self
    }
}

// impl From<Task> for CreateTaskBody {
//     fn from(task: Task) -> CreateTaskBody {
//         unimplemented!();
//     }
// }

#[derive(Serialize, Debug)]
pub struct UpdateTaskBody {
    content: Option<String>,
    description: Option<String>,
    labels: Option<Vec<String>>,
    priority: Option<isize>,
    due_string: Option<String>,
    due_date: Option<String>,
    due_datetime: Option<String>,
    due_lang: Option<String>,
    assignee_id: Option<String>,
}

impl UpdateTaskBody {
    pub fn new() -> UpdateTaskBody {
        UpdateTaskBody {
            content: None,
            description: None,
            labels: None,
            priority: None,
            due_string: None,
            due_date: None,
            due_datetime: None,
            due_lang: None,
            assignee_id: None,
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = Some(content.to_string());
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    pub fn set_labels(&mut self, labels: Vec<&str>) {
        self.labels = Some(labels.iter().map(|s| s.to_string()).collect());
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = Some(priority as isize);
    }

    pub fn set_due_string(&mut self, due_string: &str) {
        self.due_string = Some(due_string.to_string());

        if self.due_date.is_some() {
            self.due_date = None;
        }
        if self.due_datetime.is_some() {
            self.due_datetime = None;
        }
    }

    pub fn set_due_date(&mut self, due_date: &str) {
        self.due_date = Some(due_date.to_string());

        if self.due_string.is_some() {
            self.due_string = None;
        }
        if self.due_datetime.is_some() {
            self.due_datetime = None;
        }
    }

    pub fn set_due_datetime(&mut self, due_datetime: &str) {
        self.due_datetime = Some(due_datetime.to_string());

        if self.due_string.is_some() {
            self.due_string = None;
        }
        if self.due_date.is_some() {
            self.due_date = None;
        }
    }

    pub fn set_due_lang(&mut self, due_lang: &str) {
        self.due_lang = Some(due_lang.to_string());
    }

    pub fn set_assignee_id(&mut self, assignee_id: &str) {
        self.assignee_id = Some(assignee_id.to_string());
    }

    pub fn with_content(mut self, content: &str) -> UpdateTaskBody {
        self.set_content(content);
        self
    }

    pub fn with_description(mut self, description: &str) -> UpdateTaskBody {
        self.set_description(description);
        self
    }

    pub fn with_labels(mut self, labels: Vec<&str>) -> UpdateTaskBody {
        self.set_labels(labels);
        self
    }

    pub fn with_priority(mut self, priority: u8) -> UpdateTaskBody {
        self.set_priority(priority);
        self
    }

    pub fn with_due_string(mut self, due_string: &str) -> UpdateTaskBody {
        self.set_due_string(due_string);
        self
    }

    pub fn with_due_date(mut self, due_date: &str) -> UpdateTaskBody {
        self.set_due_date(due_date);
        self
    }

    pub fn with_due_datetime(mut self, due_datetime: &str) -> UpdateTaskBody {
        self.set_due_datetime(due_datetime);
        self
    }
}

// impl From<Task> for UpdateTaskBody {
//     fn from(task: Task) -> UpdateTaskBody {
//         unimplemented!();
//     }
// }
