use serde_json::{json, Value};

use crate::rest::models::{Attachment, Comment};

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

impl From<&Comment> for CreateCommentBody {
    fn from(comment: &Comment) -> CreateCommentBody {
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
}
