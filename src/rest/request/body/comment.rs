use serde::Serialize;

use crate::rest::models::{Attachment, Comment};

#[derive(Serialize, Debug)]
pub struct CreateProjectCommentBody {
    content: String,
    project_id: String,
    attachment: Option<Attachment>,
}

impl CreateProjectCommentBody {
    pub fn new(content: &str, project_id: &str) -> CreateProjectCommentBody {
        CreateProjectCommentBody {
            content: content.to_string(),
            project_id: project_id.to_string(),
            attachment: None,
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    pub fn set_project_id(&mut self, project_id: &str) {
        self.project_id = project_id.to_string();
    }

    pub fn set_attachment(&mut self, attachment: Attachment) {
        self.attachment = Some(attachment);
    }

    pub fn with_content(mut self, content: &str) -> CreateProjectCommentBody {
        self.set_content(content);
        self
    }

    pub fn with_project_id(mut self, project_id: &str) -> CreateProjectCommentBody {
        self.set_project_id(project_id);
        self
    }

    pub fn with_attachment(mut self, attachment: Attachment) -> CreateProjectCommentBody {
        self.set_attachment(attachment);
        self
    }
}

impl From<&Comment> for CreateProjectCommentBody {
    fn from(comment: &Comment) -> Self {
        let attachment = if comment.attachment.is_some() {
            Some(comment.attachment.clone().unwrap())
        } else {
            None
        };

        CreateProjectCommentBody {
            content: comment.content.clone(),
            project_id: comment.project_id.clone().unwrap(),
            attachment,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CreateTaskCommentBody {
    content: String,
    task_id: String,
    attachment: Option<Attachment>,
}

impl CreateTaskCommentBody {
    pub fn new(content: &str, task_id: &str) -> CreateTaskCommentBody {
        CreateTaskCommentBody {
            content: content.to_string(),
            task_id: task_id.to_string(),
            attachment: None,
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    pub fn set_task_id(&mut self, task_id: &str) {
        self.task_id = task_id.to_string();
    }

    pub fn set_attachment(&mut self, attachment: Attachment) {
        self.attachment = Some(attachment);
    }

    pub fn with_content(mut self, content: &str) -> CreateTaskCommentBody {
        self.set_content(content);
        self
    }

    pub fn with_task_id(mut self, task_id: &str) -> CreateTaskCommentBody {
        self.set_task_id(task_id);
        self
    }

    pub fn with_attachment(mut self, attachment: Attachment) -> CreateTaskCommentBody {
        self.set_attachment(attachment);
        self
    }
}
