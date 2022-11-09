use crate::{
    error::TodorstError,
    rest::{
        body::{CreateCommentBody, CreateTaskBody, UpdateTaskBody},
        TodorstRestAPI,
    },
};

use super::{CommentAPI, Due, Priority, ProjectAPI, SectionAPI};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub creator_id: String,

    pub created_at: String,

    pub assignee_id: Option<String>,

    pub assigner_id: Option<String>,

    pub comment_count: isize,

    pub is_completed: bool,

    pub content: String,

    pub description: String,

    pub due: Option<Due>,

    pub id: String,

    pub labels: Vec<String>,

    pub order: isize,

    pub priority: Priority,

    pub project_id: Option<String>,

    pub section_id: Option<String>,

    pub parent_id: Option<String>,

    pub url: String,
}

#[derive(Debug, Clone)]
pub struct TaskAPI<'a> {
    api: &'a TodorstRestAPI<'a>,
    task: Task,
}

impl<'a> TaskAPI<'a> {
    pub fn new(api: &'a TodorstRestAPI, task: Task) -> Self {
        Self { api, task }
    }

    pub fn get(&self) -> &Task {
        &self.task
    }

    #[maybe_async::maybe_async]
    pub async fn update(&self, body: UpdateTaskBody) -> Result<TaskAPI, TodorstError> {
        Ok(self.api.update_task(&self.task.id, body).await?)
    }

    #[maybe_async::maybe_async]
    pub async fn close(&self) -> Result<(), TodorstError> {
        self.api.close_task(&self.task.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn reopen(&self) -> Result<(), TodorstError> {
        self.api.reopen_task(&self.task.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn delete(&self) -> Result<(), TodorstError> {
        self.api.delete_task(&self.task.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn get_parent(&self) -> Result<Option<TaskAPI>, TodorstError> {
        if let Some(parent_id) = &self.task.parent_id {
            Ok(Some(self.api.get_task(parent_id).await?))
        } else {
            Ok(None)
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_project(&self) -> Result<Option<ProjectAPI>, TodorstError> {
        if let Some(project_id) = &self.task.project_id {
            Ok(Some(self.api.get_project(project_id).await?))
        } else {
            Ok(None)
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_section(&self) -> Result<Option<SectionAPI>, TodorstError> {
        if let Some(section_id) = &self.task.section_id {
            Ok(Some(self.api.get_section(section_id).await?))
        } else {
            Ok(None)
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_comments(&self) -> Result<Vec<CommentAPI>, TodorstError> {
        self.api.get_task_comments(&self.task.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn add_comment(&self, body: CreateCommentBody) -> Result<CommentAPI, TodorstError> {
        self.api
            .create_comment(body.with_task_id(&self.task.id))
            .await
    }

    #[maybe_async::maybe_async]
    pub async fn create_child_task(&self, body: CreateTaskBody) -> Result<TaskAPI, TodorstError> {
        self.api
            .create_task(body.with_parent_id(&self.task.id))
            .await
    }
}
