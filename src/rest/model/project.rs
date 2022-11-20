use crate::{
    error::TodorstError,
    rest::{
        body::{
            CreateProjectBody, CreateProjectCommentBody, CreateSectionBody, CreateTaskBody,
            UpdateProjectBody,
        },
        request::query::GetTasksQuery,
        TodorstRestAPI,
    },
    types::Color
};

use super::{Attachment, Collaborator, CommentAPI, SectionAPI, TaskAPI, ViewStyle};
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,

    pub name: String,

    pub color: Color,

    pub parent_id: Option<String>,

    pub order: isize,

    pub comment_count: isize,

    pub is_shared: bool,

    pub is_favorite: bool,

    pub is_inbox_project: bool,

    pub is_team_inbox: bool,

    pub view_style: ViewStyle,

    pub url: String,
}

#[derive(Debug, Clone)]
pub struct ProjectAPI<'a> {
    api: &'a TodorstRestAPI<'a>,
    project: Project,
}

impl<'a> ProjectAPI<'a> {
    pub fn new(api: &'a TodorstRestAPI<'a>, project: Project) -> ProjectAPI<'a> {
        ProjectAPI { api, project }
    }

    pub fn from_iter(
        api: &'a TodorstRestAPI<'a>,
        projects: impl Iterator<Item = Project>,
    ) -> Vec<Self> {
        projects
            .into_iter()
            .map(|project| Self::new(api, project))
            .collect()
    }

    pub fn get(&self) -> &Project {
        &self.project
    }

    #[maybe_async::maybe_async]
    pub async fn update(&self, body: UpdateProjectBody) -> Result<ProjectAPI, TodorstError> {
        self.api.update_project(&self.project.id, body).await
    }

    #[maybe_async::maybe_async]
    pub async fn delete(&self) -> Result<Response, TodorstError> {
        self.api.delete_project(&self.project.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn get_parent(&self) -> Result<Option<ProjectAPI>, TodorstError> {
        if let Some(parent_id) = &self.project.parent_id {
            Ok(Some(self.api.get_project(parent_id).await?))
        } else {
            Ok(None)
        }
    }

    #[maybe_async::maybe_async]
    pub async fn create_child(&self, body: CreateProjectBody) -> Result<ProjectAPI, TodorstError> {
        self.api
            .create_project(body.with_parent_id(&self.project.id))
            .await
    }

    #[maybe_async::maybe_async]
    pub async fn get_collaborators(&self) -> Result<Vec<Collaborator>, TodorstError> {
        self.api.get_collaborators(&self.project.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn get_tasks(&self, query: GetTasksQuery) -> Result<Vec<TaskAPI>, TodorstError> {
        self.api
            .get_tasks_with_query(query.with_project_id(&self.project.id))
            .await
    }

    #[maybe_async::maybe_async]
    pub async fn create_task(&self, body: CreateTaskBody) -> Result<TaskAPI, TodorstError> {
        self.api
            .create_task(body.with_project_id(&self.project.id))
            .await
    }

    #[maybe_async::maybe_async]
    pub async fn get_sections(&self) -> Result<Vec<SectionAPI>, TodorstError> {
        self.api.get_sections(&self.project.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn create_section(&self, name: &str) -> Result<SectionAPI, TodorstError> {
        self.api
            .create_section(CreateSectionBody::new(name, &self.project.id))
            .await
    }

    #[maybe_async::maybe_async]
    pub async fn create_section_with_order(
        &self,
        name: &str,
        order: isize,
    ) -> Result<SectionAPI, TodorstError> {
        self.api
            .create_section(CreateSectionBody::new(name, &self.project.id).with_order(order))
            .await
    }

    #[maybe_async::maybe_async]
    pub async fn get_comments(&self) -> Result<Vec<CommentAPI>, TodorstError> {
        self.api.get_project_comments(&self.project.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn create_comment(&self, content: &str) -> Result<CommentAPI, TodorstError> {
        self.api
            .create_project_comment(CreateProjectCommentBody::new(&self.project.id, content))
            .await
    }

    #[maybe_async::maybe_async]
    pub async fn create_comment_with_attachment(
        &self,
        content: &str,
        attachment: Attachment,
    ) -> Result<CommentAPI, TodorstError> {
        self.api
            .create_project_comment(
                CreateProjectCommentBody::new(&self.project.id, content)
                    .with_attachment(attachment),
            )
            .await
    }
}
