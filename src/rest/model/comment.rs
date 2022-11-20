use reqwest::Response;
use serde::{Deserialize, Serialize};

use crate::{error::TodorstError, rest::TodorstRestAPI};

use super::{attachment::Attachment, ProjectAPI, TaskAPI};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub content: String,

    pub id: String,

    pub posted_at: String,

    pub project_id: Option<String>,

    pub task_id: Option<String>,

    pub attachment: Option<Attachment>,
}

#[derive(Debug, Clone)]
pub struct CommentAPI<'a> {
    api: &'a TodorstRestAPI<'a>,
    comment: Comment,
}

impl<'a> CommentAPI<'a> {
    pub fn new(api: &'a TodorstRestAPI<'a>, comment: Comment) -> Self {
        Self { api, comment }
    }

    pub fn from_iter(
        api: &'a TodorstRestAPI<'a>,
        comments: impl Iterator<Item = Comment>,
    ) -> Vec<Self> {
        comments
            .into_iter()
            .map(|comment| Self::new(api, comment))
            .collect()
    }

    pub fn get(&self) -> &Comment {
        &self.comment
    }

    #[maybe_async::maybe_async]
    pub async fn update(&mut self, content: &str) -> Result<CommentAPI, TodorstError> {
        Ok(self.api.update_comment(&self.comment.id, content).await?)
    }

    #[maybe_async::maybe_async]
    pub async fn delete(&self) -> Result<Response, TodorstError> {
        self.api.delete_comment(&self.comment.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn get_task(&self) -> Result<Option<TaskAPI>, TodorstError> {
        if let Some(task_id) = &self.comment.task_id {
            Ok(Some(self.api.get_task(task_id).await?))
        } else {
            Ok(None)
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_project(&self) -> Result<Option<ProjectAPI>, TodorstError> {
        if let Some(project_id) = &self.comment.project_id {
            Ok(Some(self.api.get_project(project_id).await?))
        } else {
            Ok(None)
        }
    }
}
