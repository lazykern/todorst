use serde_json::json;

use super::body::*;
use super::models::*;
use super::request::endpoint::*;
use super::request::query::GetTasksQuery;
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
        let response = self.client.post(url).json(body.json()).send().await?;
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
    pub async fn get_all_sections(&self) -> Result<Vec<Section>, TodorstError> {
        let url = rest_sections_url();
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_sections(&self, project_id: &str) -> Result<Vec<Section>, TodorstError> {
        let url = rest_sections_url();
        let query = [("project_id", project_id)];
        let response = self.client.get(url).query(&query).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn create_section(&self, body: &CreateSectionBody) -> Result<Section, TodorstError> {
        let url = rest_sections_url();
        let response = self.client.post(url).json(body.json()).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_section(&self, section_id: &str) -> Result<Section, TodorstError> {
        let url = rest_section_url(section_id);
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn update_section(
        &self,
        section_id: &str,
        body: &CreateSectionBody,
    ) -> Result<Section, TodorstError> {
        let url = rest_section_url(section_id);
        let response = self.client.post(url).json(body.json()).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn delete_section(&self, section_id: &str) -> Result<(), TodorstError> {
        let url = rest_section_url(section_id);
        let response = self.client.delete(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_tasks(&self) -> Result<Vec<Task>, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_tasks_query(&self, query: GetTasksQuery) -> Result<Vec<Task>, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.get(url).query(&query).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn create_task(&self, body: &CreateTaskBody) -> Result<Task, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.post(url).json(body.json()).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_task(&self, task_id: &str) -> Result<Task, TodorstError> {
        let url = rest_task_url(task_id);
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn update_task(
        &self,
        task_id: &str,
        body: &UpdateTaskBody,
    ) -> Result<Task, TodorstError> {
        let url = rest_task_url(task_id);
        let response = self.client.post(url).json(body.json()).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn close_task(&self, task_id: &str) -> Result<(), TodorstError> {
        let url = rest_task_close_url(task_id);
        let response = self.client.post(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn reopen_task(&self, task_id: &str) -> Result<(), TodorstError> {
        let url = rest_task_reopen_url(task_id);
        let response = self.client.post(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn delete_task(&self, task_id: &str) -> Result<(), TodorstError> {
        let url = rest_task_url(task_id);
        let response = self.client.delete(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_task_comments(&self, task_id: &str) -> Result<Vec<Comment>, TodorstError> {
        let url = rest_task_comments_url(task_id);
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_project_comments(
        &self,
        project_id: &str,
    ) -> Result<Vec<Comment>, TodorstError> {
        let url = rest_project_comments_url(project_id);
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_comment(&self, comment_id: &str) -> Result<Comment, TodorstError> {
        let url = rest_comment_url(comment_id);
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn update_comment(
        &self,
        comment_id: &str,
        content: &str,
    ) -> Result<Comment, TodorstError> {
        let url = rest_comment_url(comment_id);
        let json = json!({ "content": content });
        let response = self.client.post(url).json(&json).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn delete_comment(&self, comment_id: &str) -> Result<(), TodorstError> {
        let url = rest_comment_url(comment_id);
        let response = self.client.delete(url).send().await?;
        Ok(response.json().await?)
    }
}
