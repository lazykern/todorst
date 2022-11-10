use serde_json::json;

use super::body::*;
use super::models::*;
use super::request::endpoint::*;
use super::request::query::GetTasksQuery;
use crate::client;
use crate::error::TodorstError;

#[derive(Debug, Clone)]
pub struct TodorstRestAPI<'a> {
    client: &'a client::Client,
}

impl TodorstRestAPI<'_> {
    pub fn new(client: &client::Client) -> TodorstRestAPI {
        TodorstRestAPI { client }
    }

    #[maybe_async::maybe_async]
    pub async fn get_projects(&self) -> Result<Vec<ProjectAPI>, TodorstError> {
        let url = rest_projects_url();
        let response = self.client.get(url).send().await?;
        let projects: Vec<Project> = response.json().await?;
        Ok(projects
            .into_iter()
            .map(|project| ProjectAPI::new(self, project))
            .collect())
    }

    #[maybe_async::maybe_async]
    pub async fn create_project(
        &self,
        body: CreateProjectBody,
    ) -> Result<ProjectAPI, TodorstError> {
        let url = rest_projects_url();
        let response = self.client.post(url).json(&body).send().await?;
        Ok(ProjectAPI::new(self, response.json().await?))
    }

    #[maybe_async::maybe_async]
    pub async fn get_project(&self, project_id: &str) -> Result<ProjectAPI, TodorstError> {
        let url = rest_project_url(project_id);
        let response = self.client.get(url).send().await?;
        Ok(ProjectAPI::new(self, response.json().await?))
    }

    #[maybe_async::maybe_async]
    pub async fn update_project(
        &self,
        project_id: &str,
        body: UpdateProjectBody,
    ) -> Result<ProjectAPI, TodorstError> {
        let url = rest_project_url(project_id);
        let response = self.client.post(url).json(&body).send().await?;
        Ok(ProjectAPI::new(&self, response.json().await?))
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
    pub async fn get_all_sections(&self) -> Result<Vec<SectionAPI>, TodorstError> {
        let url = rest_sections_url();
        let response = self.client.get(url).send().await?;
        let sections: Vec<Section> = response.json().await?;
        Ok(sections
            .into_iter()
            .map(|section| SectionAPI::new(&self, section))
            .collect())
    }

    #[maybe_async::maybe_async]
    pub async fn get_sections(&self, project_id: &str) -> Result<Vec<SectionAPI>, TodorstError> {
        let url = rest_sections_url();
        let query = [("project_id", project_id)];
        let response = self.client.get(url).query(&query).send().await?;
        let sections: Vec<Section> = response.json().await?;
        Ok(sections
            .into_iter()
            .map(|section| SectionAPI::new(&self, section))
            .collect())
    }

    #[maybe_async::maybe_async]
    pub async fn create_section(
        &self,
        body: CreateSectionBody,
    ) -> Result<SectionAPI, TodorstError> {
        let url = rest_sections_url();
        let response = self.client.post(url).json(&body).send().await?;
        Ok(SectionAPI::new(&self, response.json().await?))
    }

    #[maybe_async::maybe_async]
    pub async fn get_section(&self, section_id: &str) -> Result<SectionAPI, TodorstError> {
        let url = rest_section_url(section_id);
        let response = self.client.get(url).send().await?;
        Ok(SectionAPI::new(&self, response.json().await?))
    }

    #[maybe_async::maybe_async]
    pub async fn update_section(
        &self,
        section_id: &str,
        name: &str,
    ) -> Result<SectionAPI, TodorstError> {
        let url = rest_section_url(section_id);
        let body = json!({ "name": name });
        let response = self.client.post(url).json(&body).send().await?;
        Ok(SectionAPI::new(&self, response.json().await?))
    }

    #[maybe_async::maybe_async]
    pub async fn delete_section(&self, section_id: &str) -> Result<(), TodorstError> {
        let url = rest_section_url(section_id);
        let response = self.client.delete(url).send().await?;
        Ok(response.json().await?)
    }

    #[maybe_async::maybe_async]
    pub async fn get_tasks(&self) -> Result<Vec<TaskAPI>, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.get(url).send().await?;
        let tasks: Vec<Task> = response.json().await?;
        Ok(tasks.into_iter().map(|t| TaskAPI::new(&self, t)).collect())
    }

    #[maybe_async::maybe_async]
    pub async fn get_tasks_with_query(
        &self,
        query: GetTasksQuery,
    ) -> Result<Vec<TaskAPI>, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.get(url).query(&query).send().await?;
        let tasks: Vec<Task> = response.json().await?;
        Ok(tasks.into_iter().map(|t| TaskAPI::new(&self, t)).collect())
    }

    #[maybe_async::maybe_async]
    pub async fn create_task(&self, body: CreateTaskBody) -> Result<TaskAPI, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.post(url).json(&body).send().await?;
        Ok(TaskAPI::new(&self, response.json().await?))
    }

    #[maybe_async::maybe_async]
    pub async fn get_task(&self, task_id: &str) -> Result<TaskAPI, TodorstError> {
        let url = rest_task_url(task_id);
        let response = self.client.get(url).send().await?;
        Ok(TaskAPI::new(&self, response.json().await?))
    }

    #[maybe_async::maybe_async]
    pub async fn update_task(
        &self,
        task_id: &str,
        body: UpdateTaskBody,
    ) -> Result<TaskAPI, TodorstError> {
        let url = rest_task_url(task_id);
        let response = self.client.post(url).json(&body).send().await?;
        Ok(TaskAPI::new(&self, response.json().await?))
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
    pub async fn get_task_comments(&self, task_id: &str) -> Result<Vec<CommentAPI>, TodorstError> {
        let url = rest_task_comments_url(task_id);
        let response = self.client.get(url).send().await?;
        let comments: Vec<Comment> = response.json().await?;
        Ok(comments
            .into_iter()
            .map(|c| CommentAPI::new(&self, c))
            .collect())
    }

    #[maybe_async::maybe_async]
    pub async fn get_project_comments(
        &self,
        project_id: &str,
    ) -> Result<Vec<CommentAPI>, TodorstError> {
        let url = rest_project_comments_url(project_id);
        let response = self.client.get(url).send().await?;
        let comments: Vec<Comment> = response.json().await?;
        Ok(comments
            .into_iter()
            .map(|c| CommentAPI::new(&self, c))
            .collect())
    }

    #[maybe_async::maybe_async]
    pub async fn create_comment(
        &self,
        body: CreateCommentBody,
    ) -> Result<CommentAPI, TodorstError> {
        let url = rest_comments_url();
        let response = self.client.post(url).json(&body).send().await?;
        Ok(CommentAPI::new(&self, response.json().await?))
    }

    #[maybe_async::maybe_async]
    pub async fn get_comment(&self, comment_id: &str) -> Result<CommentAPI, TodorstError> {
        let url = rest_comment_url(comment_id);
        let response = self.client.get(url).send().await?;
        Ok(CommentAPI::new(&self, response.json().await?))
    }

    #[maybe_async::maybe_async]
    pub async fn update_comment(
        &self,
        comment_id: &str,
        content: &str,
    ) -> Result<CommentAPI, TodorstError> {
        let url = rest_comment_url(comment_id);
        let json = json!({ "content": content });
        let response = self.client.post(url).json(&json).send().await?;

        Ok(CommentAPI::new(&self, response.json().await?))
    }

    #[maybe_async::maybe_async]
    pub async fn delete_comment(&self, comment_id: &str) -> Result<(), TodorstError> {
        let url = rest_comment_url(comment_id);
        let response = self.client.delete(url).send().await?;
        Ok(response.json().await?)
    }
}
