use serde_json::{json, Value};

use crate::rest::models::Task;

pub struct CreateTaskBody {
    json: Value,
}

impl CreateTaskBody {
    pub fn new(content: &str) -> CreateTaskBody {
        let json = json!({
            "content": content,
        });
        CreateTaskBody { json }
    }

    pub fn json(&self) -> &Value {
        &self.json
    }

    pub fn set_content(&mut self, content: &str) {
        self.json["content"] = json!(content);
    }

    pub fn set_description(&mut self, description: &str) {
        self.json["description"] = json!(description);
    }

    pub fn set_project_id(&mut self, project_id: &str) {
        self.json["project_id"] = json!(project_id);
    }

    pub fn set_section_id(&mut self, section_id: &str) {
        self.json["section_id"] = json!(section_id);
    }

    pub fn set_parent_id(&mut self, parent_id: &str) {
        self.json["parent_id"] = json!(parent_id);
    }

    pub fn set_order(&mut self, order: isize) {
        self.json["order"] = json!(order);
    }

    pub fn set_labels(&mut self, labels: Vec<&str>) {
        self.json["labels"] = json!(labels);
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.json["priority"] = json!(priority);
    }

    pub fn set_due_string(&mut self, due_string: &str) {
        if self.json.get("due_date").is_some() {
            self.json["due_date"].take();
        }
        if self.json.get("due_datetime").is_some() {
            self.json["due_datetime"].take();
        }
        self.json["due_string"] = json!(due_string);
    }

    pub fn set_due_date(&mut self, due_date: &str) {
        if self.json.get("due_string").is_some() {
            self.json["due_string"].take();
        }
        if self.json.get("due_datetime").is_some() {
            self.json["due_datetime"].take();
        }
        self.json["due_date"] = json!(due_date);
    }

    pub fn set_due_datetime(&mut self, due_datetime: &str) {
        if self.json.get("due_string").is_some() {
            self.json["due_string"].take();
        }
        if self.json.get("due_date").is_some() {
            self.json["due_date"].take();
        }
        self.json["due_datetime"] = json!(due_datetime);
    }

    pub fn set_due_lang(&mut self, due_lang: &str) {
        self.json["due_lang"] = json!(due_lang);
    }

    pub fn set_assignee_id(&mut self, assignee_id: &str) {
        self.json["assignee_id"] = json!(assignee_id);
    }

    pub fn with_description(mut self, description: &str) -> CreateTaskBody {
        self.json["description"] = json!(description);
        self
    }

    pub fn with_project_id(mut self, project_id: &str) -> CreateTaskBody {
        self.json["project_id"] = json!(project_id);
        self
    }

    pub fn with_section_id(mut self, section_id: &str) -> CreateTaskBody {
        self.json["section_id"] = json!(section_id);
        self
    }

    pub fn with_parent_id(mut self, parent_id: &str) -> CreateTaskBody {
        self.json["parent_id"] = json!(parent_id);
        self
    }

    pub fn with_order(mut self, order: isize) -> CreateTaskBody {
        self.json["order"] = json!(order);
        self
    }

    pub fn with_labels(mut self, labels: Vec<&str>) -> CreateTaskBody {
        self.json["labels"] = json!(labels);
        self
    }

    pub fn with_priority(mut self, priority: u8) -> CreateTaskBody {
        self.json["priority"] = json!(priority);
        self
    }

    pub fn with_due_string(mut self, due_string: &str) -> CreateTaskBody {
        if self.json.get("due_date").is_some() {
            self.json["due_date"].take();
        }
        if self.json.get("due_datetime").is_some() {
            self.json["due_datetime"].take();
        }
        self.json["due_string"] = json!(due_string);
        self
    }

    pub fn with_due_date(mut self, due_date: &str) -> CreateTaskBody {
        if self.json.get("due_string").is_some() {
            self.json["due_string"].take();
        }
        if self.json.get("due_datetime").is_some() {
            self.json["due_datetime"].take();
        }
        self.json["due_date"] = json!(due_date);
        self
    }

    pub fn with_due_datetime(mut self, due_datetime: &str) -> CreateTaskBody {
        if self.json.get("due_string").is_some() {
            self.json["due_string"].take();
        }
        if self.json.get("due_date").is_some() {
            self.json["due_date"].take();
        }
        self.json["due_datetime"] = json!(due_datetime);
        self
    }

    pub fn with_due_lang(mut self, due_lang: &str) -> CreateTaskBody {
        self.json["due_lang"] = json!(due_lang);
        self
    }

    pub fn with_assignee_id(mut self, assignee_id: &str) -> CreateTaskBody {
        self.json["assignee_id"] = json!(assignee_id);
        self
    }
}

impl From<Task> for CreateTaskBody {
    fn from(task: Task) -> CreateTaskBody {
        let mut json = json!({
            "content": task.content,
            "description": task.description,
            "project_id": task.project_id,
            "section_id": task.section_id,
            "parent_id": task.parent_id,
            "order": task.order,
            "labels": task.labels,
            "priority": task.priority,
            "assignee_id": task.assignee_id,
        });
        if let Some(due) = &task.due {
            // TODO
            json["due_string"] = json!(due.string);
        }

        CreateTaskBody { json }
    }
}

pub struct UpdateTaskBody {
    json: Value,
}

impl UpdateTaskBody {
    pub fn new() -> UpdateTaskBody {
        let json = json!({});
        UpdateTaskBody { json }
    }

    pub fn json(&self) -> &Value {
        &self.json
    }

    pub fn set_content(&mut self, content: &str) {
        self.json["content"] = json!(content);
    }

    pub fn set_description(&mut self, description: &str) {
        self.json["description"] = json!(description);
    }

    pub fn set_labels(&mut self, labels: Vec<&str>) {
        self.json["labels"] = json!(labels);
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.json["priority"] = json!(priority);
    }

    pub fn set_due_string(&mut self, due_string: &str) {
        if self.json.get("due_date").is_some() {
            self.json["due_date"].take();
        }
        if self.json.get("due_datetime").is_some() {
            self.json["due_datetime"].take();
        }
        self.json["due_string"] = json!(due_string);
    }

    pub fn set_due_date(&mut self, due_date: &str) {
        if self.json.get("due_string").is_some() {
            self.json["due_string"].take();
        }
        if self.json.get("due_datetime").is_some() {
            self.json["due_datetime"].take();
        }
        self.json["due_date"] = json!(due_date);
    }

    pub fn set_due_datetime(&mut self, due_datetime: &str) {
        if self.json.get("due_string").is_some() {
            self.json["due_string"].take();
        }
        if self.json.get("due_date").is_some() {
            self.json["due_date"].take();
        }
        self.json["due_datetime"] = json!(due_datetime);
    }

    pub fn set_due_lang(&mut self, due_lang: &str) {
        self.json["due_lang"] = json!(due_lang);
    }

    pub fn set_assignee_id(&mut self, assignee_id: &str) {
        self.json["assignee_id"] = json!(assignee_id);
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

impl From<Task> for UpdateTaskBody {
    fn from(task: Task) -> UpdateTaskBody {
        let mut json = json!({
            "description": task.description,
            "project_id": task.project_id,
            "section_id": task.section_id,
            "parent_id": task.parent_id,
            "order": task.order,
            "labels": task.labels,
            "priority": task.priority,
            "assignee_id": task.assignee_id,
        });

        if let Some(due) = &task.due {
            // TODO
            json["due_string"] = json!(due.string);
        }

        UpdateTaskBody { json }
    }
}
