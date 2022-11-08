use serde_json::{json, Value};

use crate::rest::models::*;

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

    pub fn from_project(project: &Project) -> CreateProjectBody {
        let json = json!({
            "name": project.name,
            "color": project.color,
            "parent_id": project.parent_id,
            "order": project.order,
            "comment_count": project.comment_count,
            "is_shared": project.is_shared,
            "is_favorite": project.is_favorite,
            "is_inbox_project": project.is_inbox_project,
            "is_team_inbox": project.is_team_inbox,
            "view_style": project.view_style,
            "url": project.url,
        });
        CreateProjectBody { json }
    }

    pub fn json(&self) -> &Value {
        &self.json
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

pub struct UpdateProjectBody {
    json: Value,
}

impl UpdateProjectBody {
    pub fn new() -> UpdateProjectBody {
        let json = json!({});
        UpdateProjectBody { json }
    }

    pub fn from_project(project: &Project) -> UpdateProjectBody {
        let json = json!({
            "name": project.name,
            "color": project.color,
            "parent_id": project.parent_id,
            "order": project.order,
            "comment_count": project.comment_count,
            "is_shared": project.is_shared,
            "is_favorite": project.is_favorite,
            "is_inbox_project": project.is_inbox_project,
            "is_team_inbox": project.is_team_inbox,
            "view_style": project.view_style,
            "url": project.url,
        });
        UpdateProjectBody { json }
    }

    pub fn json(&self) -> &Value {
        &self.json
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

pub struct UpdateTaskBody {
    json: Value,
}

impl UpdateTaskBody {
    pub fn new() -> UpdateTaskBody {
        let json = json!({});
        UpdateTaskBody { json }
    }

    pub fn from_task(task: &Task) -> UpdateTaskBody {
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

    pub fn from_comment(comment: &Comment) -> CreateCommentBody {
        let mut json = json!({
            "content": comment.content,
        });

        if let Some(task_id) = &comment.task_id {
            json["task_id"] = json!(task_id);
        }

        if let Some(project_id) = &comment.project_id {
            json["project_id"] = json!(project_id);
        }

        if let Some(attachment) = &comment.attachment {
            json["attachment"] = json!(attachment);
        }

        CreateCommentBody { json }
    }

    pub fn json(&self) -> &Value {
        &self.json
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

    pub fn with_content(mut self, content: &str) -> CreateCommentBody {
        self.set_content(content);
        self
    }

    pub fn with_task_id(mut self, task_id: &str) -> CreateCommentBody {
        self.set_task_id(task_id);
        self
    }

    pub fn with_project_id(mut self, project_id: &str) -> CreateCommentBody {
        self.set_project_id(project_id);
        self
    }

    pub fn with_attachment(mut self, attachment: Attachment) -> CreateCommentBody {
        self.set_attatchment(attachment);
        self
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

    pub fn from_comment(comment: &Comment) -> UpdateCommentBody {
        let json = json!({
            "content": comment.content,
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

    pub fn from_label(label: &Label) -> CratePersonalLabelBody {
        let json = json!({
            "name": label.name,
            "color": label.color,
            "order": label.order,
            "is_favorite": label.is_favorite,
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

pub type UpdatePersonalLabelBody = CratePersonalLabelBody;
