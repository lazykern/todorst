use super::body::{CreateProjectBody, UpdateProjectBody};
use super::models::*;
use super::request::endpoint::*;
use crate::client;
use crate::error::TodorstError;

pub struct TodorstRestApi<'a> {
    client: &'a client::Client,
}

impl TodorstRestApi<'_> {
    pub fn new(client: &client::Client) -> TodorstRestApi {
        TodorstRestApi { client }
    }

    #[maybe_async::maybe_async]
    pub async fn get_projects(&self) -> Result<Vec<Project>, TodorstError> {
        let url = rest_projects_url();
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn crate_project(&self, body: &CreateProjectBody) -> Result<Project, TodorstError> {
        let url = rest_projects_url();
        let response = self.client.post(url).json(body.json()).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_project(&self, project_id: &str) -> Result<Project, TodorstError> {
        let url = rest_project_url(project_id);
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn update_project(
        &self,
        project_id: &str,
        body: &UpdateProjectBody,
    ) -> Result<Project, TodorstError> {
        let url = rest_project_url(project_id);
        let response = self.client.patch(url).json(body.json()).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn delete_project(&self, project_id: &str) -> Result<(), TodorstError> {
        let url = rest_project_url(project_id);
        let response = self.client.delete(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_collaborators(
        &self,
        project_id: &str,
    ) -> Result<Vec<Collaborator>, TodorstError> {
        let url = rest_collaborators_url(project_id);
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_tasks(&self) -> Result<Vec<Task>, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }
}
