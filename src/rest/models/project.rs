use crate::{
    error::TodorstError,
    rest::{body::UpdateProjectBody, TodorstRestAPI},
};

use super::{Collaborator, Color, SectionAPI};
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

    pub view_style: String,

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

    pub fn get(&self) -> &Project {
        &self.project
    }

    #[maybe_async::maybe_async]
    pub async fn update(&self, body: UpdateProjectBody) -> Result<ProjectAPI, TodorstError> {
        self.api.update_project(&self.project.id, body).await
    }

    #[maybe_async::maybe_async]
    pub async fn delete(&self) -> Result<(), TodorstError> {
        self.api.delete_project(&self.project.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn get_collaborators(&self) -> Result<Vec<Collaborator>, TodorstError> {
        self.api.get_collaborators(&self.project.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn get_sections(&self) -> Result<Vec<SectionAPI>, TodorstError> {
        self.api.get_sections(&self.project.id).await
    }
}
