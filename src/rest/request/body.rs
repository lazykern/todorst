use serde_json::{json, Value};

use crate::rest::models::{Attachment, Color};

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
}

pub struct CreateCommentBody {
    json: Value,
}

impl CreateCommentBody {
    pub fn new(content: &str) -> CreateCommentBody {
        let json = json!({
            "content": content,
        });
        CreateCommentBody { json }
    }

    pub fn new_with_task_id(task_id: &str, content: &str) -> CreateCommentBody {
        let json = json!({
            "task_id": task_id,
            "content": content,
        });
        CreateCommentBody { json }
    }

    pub fn new_with_project_id(project_id: &str, content: &str) -> CreateCommentBody {
        let json = json!({
            "project_id": project_id,
            "content": content,
        });
        CreateCommentBody { json }
    }

    pub fn json(&self) -> &Value {
        &self.json
    }

    pub fn set_content(&mut self, content: &str) {
        self.json["content"] = json!(content);
    }

    pub fn set_task_id(&mut self, task_id: &str) {
        if self.json.get("project_id").is_some() {
            self.json["project_id"].take();
        }
        self.json["task_id"] = json!(task_id);
    }

    pub fn set_project_id(&mut self, project_id: &str) {
        if self.json.get("task_id").is_some() {
            self.json["task_id"].take();
        }
        self.json["project_id"] = json!(project_id);
    }

    pub fn set_attatchment(&mut self, attachment: Attachment) {
        self.json["attachment"] = json!(attachment);
    }
}

pub struct UpdateCommentBody {
    json: Value,
}

impl UpdateCommentBody {
    pub fn new(content: &str) -> UpdateCommentBody {
        let json = json!({
            "content": content,
        });
        UpdateCommentBody { json }
    }

    pub fn json(&self) -> &Value {
        &self.json
    }

    pub fn set_content(&mut self, content: &str) {
        self.json["content"] = json!(content);
    }
}

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

    pub fn json(&self) -> &Value {
        &self.json
    }

    pub fn set_name(&mut self, name: &str) {
        self.json["name"] = json!(name);
    }

    pub fn set_order(&mut self, order: isize) {
        self.json["order"] = json!(order);
    }

    pub fn set_color_with_str(&mut self, color: &str) {
        self.json["color"] = json!(color);
    }

    pub fn set_color(&mut self, color: Color) {
        self.json["color"] = json!(color);
    }
}
