use reqwest::Response;
use serde_json::json;

use super::body::*;
use super::model::*;
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
        Ok(ProjectAPI::from_iter(self, projects.into_iter()))
    }

    #[maybe_async::maybe_async]
    pub async fn create_project(
        &self,
        body: CreateProjectBody,
    ) -> Result<ProjectAPI, TodorstError> {
        let url = rest_projects_url();
        let response = self.client.post(url).json(&body).send().await?;

        if response.status().is_success() {
            let project: Project = response.json().await?;
            Ok(ProjectAPI::new(self, project))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_project(&self, project_id: &str) -> Result<ProjectAPI, TodorstError> {
        let url = rest_project_url(project_id);
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let project: Project = response.json().await?;
            Ok(ProjectAPI::new(self, project))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn update_project(
        &self,
        project_id: &str,
        body: UpdateProjectBody,
    ) -> Result<ProjectAPI, TodorstError> {
        let url = rest_project_url(project_id);
        let response = self.client.post(url).json(&body).send().await?;

        if response.status().is_success() {
            let project: Project = response.json().await?;
            Ok(ProjectAPI::new(self, project))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn delete_project(&self, project_id: &str) -> Result<Response, TodorstError> {
        let url = rest_project_url(project_id);
        let response = self.client.delete(url).send().await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_collaborators(
        &self,
        project_id: &str,
    ) -> Result<Vec<Collaborator>, TodorstError> {
        let url = rest_collaborators_url(project_id);
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let collaborators: Vec<Collaborator> = response.json().await?;
            Ok(collaborators)
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_all_sections(&self) -> Result<Vec<SectionAPI>, TodorstError> {
        let url = rest_sections_url();
        let response = self.client.get(url).send().await?;
        if response.status().is_success() {
            let sections: Vec<Section> = response.json().await?;
            Ok(SectionAPI::from_iter(self, sections.into_iter()))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_sections(&self, project_id: &str) -> Result<Vec<SectionAPI>, TodorstError> {
        let url = rest_sections_url();
        let query = [("project_id", project_id)];
        let response = self.client.get(url).query(&query).send().await?;
        if response.status().is_success() {
            let sections: Vec<Section> = response.json().await?;
            Ok(SectionAPI::from_iter(self, sections.into_iter()))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn create_section(
        &self,
        body: CreateSectionBody,
    ) -> Result<SectionAPI, TodorstError> {
        let url = rest_sections_url();
        let response = self.client.post(url).json(&body).send().await?;

        if response.status().is_success() {
            let section: Section = response.json().await?;
            Ok(SectionAPI::new(self, section))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_section(&self, section_id: &str) -> Result<SectionAPI, TodorstError> {
        let url = rest_section_url(section_id);
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let section: Section = response.json().await?;
            Ok(SectionAPI::new(self, section))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
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

        if response.status().is_success() {
            let section: Section = response.json().await?;
            Ok(SectionAPI::new(self, section))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn delete_section(&self, section_id: &str) -> Result<Response, TodorstError> {
        let url = rest_section_url(section_id);
        let response = self.client.delete(url).send().await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_tasks(&self) -> Result<Vec<TaskAPI>, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let tasks: Vec<Task> = response.json().await?;
            Ok(TaskAPI::from_iter(self, tasks.into_iter()))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_tasks_with_query(
        &self,
        query: GetTasksQuery,
    ) -> Result<Vec<TaskAPI>, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.get(url).query(&query).send().await?;

        if response.status().is_success() {
            let tasks: Vec<Task> = response.json().await?;
            Ok(TaskAPI::from_iter(self, tasks.into_iter()))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn create_task(&self, body: CreateTaskBody) -> Result<TaskAPI, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.post(url).json(&body).send().await?;

        if response.status().is_success() {
            let task: Task = response.json().await?;
            Ok(TaskAPI::new(self, task))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_task(&self, task_id: &str) -> Result<TaskAPI, TodorstError> {
        let url = rest_task_url(task_id);
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let task: Task = response.json().await?;
            Ok(TaskAPI::new(self, task))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn update_task(
        &self,
        task_id: &str,
        body: UpdateTaskBody,
    ) -> Result<TaskAPI, TodorstError> {
        let url = rest_task_url(task_id);
        let response = self.client.post(url).json(&body).send().await?;

        if response.status().is_success() {
            let task: Task = response.json().await?;
            Ok(TaskAPI::new(self, task))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn close_task(&self, task_id: &str) -> Result<Response, TodorstError> {
        let url = rest_task_close_url(task_id);
        let response = self.client.post(url).send().await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn reopen_task(&self, task_id: &str) -> Result<Response, TodorstError> {
        let url = rest_task_reopen_url(task_id);
        let response = self.client.post(url).send().await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn delete_task(&self, task_id: &str) -> Result<Response, TodorstError> {
        let url = rest_task_url(task_id);
        let response = self.client.delete(url).send().await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_task_comments(&self, task_id: &str) -> Result<Vec<CommentAPI>, TodorstError> {
        let url = rest_task_comments_url(task_id);
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let comments: Vec<Comment> = response.json().await?;
            Ok(CommentAPI::from_iter(self, comments.into_iter()))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_project_comments(
        &self,
        project_id: &str,
    ) -> Result<Vec<CommentAPI>, TodorstError> {
        let url = rest_project_comments_url(project_id);
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let comments: Vec<Comment> = response.json().await?;
            Ok(CommentAPI::from_iter(self, comments.into_iter()))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn create_project_comment(
        &self,
        body: CreateProjectCommentBody,
    ) -> Result<CommentAPI, TodorstError> {
        let url = rest_comments_url();
        let response = self.client.post(url).json(&body).send().await?;

        if response.status().is_success() {
            let comment: Comment = response.json().await?;
            Ok(CommentAPI::new(&self, comment))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn create_task_comment(
        &self,
        body: CreateTaskCommentBody,
    ) -> Result<CommentAPI, TodorstError> {
        let url = rest_comments_url();
        let response = self.client.post(url).json(&body).send().await?;

        if response.status().is_success() {
            let comment: Comment = response.json().await?;
            Ok(CommentAPI::new(&self, comment))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_comment(&self, comment_id: &str) -> Result<CommentAPI, TodorstError> {
        let url = rest_comment_url(comment_id);
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let comment: Comment = response.json().await?;
            Ok(CommentAPI::new(&self, comment))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
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

        if response.status().is_success() {
            let comment: Comment = response.json().await?;
            Ok(CommentAPI::new(&self, comment))
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn delete_comment(&self, comment_id: &str) -> Result<Response, TodorstError> {
        let url = rest_comment_url(comment_id);
        let response = self.client.delete(url).send().await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_personal_labels(&self) -> Result<Vec<PersonalLabel>, TodorstError> {
        let url = rest_labels_url();
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let labels: Vec<PersonalLabel> = response.json().await?;
            Ok(labels)
        } else {
            let status = response.status();
            let body = response.text().await?;
            Err(TodorstError::Status(status, body))
        }
    }
}
