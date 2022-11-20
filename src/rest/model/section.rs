use reqwest::Response;
use serde::{Deserialize, Serialize};

use crate::{
    error::TodorstError,
    rest::{body::CreateTaskBody, request::query::GetTasksQuery, TodorstRestAPI},
};

use super::{ProjectAPI, TaskAPI};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub id: String,

    pub project_id: String,

    pub order: isize,

    pub name: String,
}

#[derive(Debug, Clone)]
pub struct SectionAPI<'a> {
    api: &'a TodorstRestAPI<'a>,
    section: Section,
}

impl<'a> SectionAPI<'a> {
    pub fn new(api: &'a TodorstRestAPI<'a>, section: Section) -> SectionAPI<'a> {
        SectionAPI { api, section }
    }

    pub fn from_iter(
        api: &'a TodorstRestAPI<'a>,
        sections: impl Iterator<Item = Section>,
    ) -> Vec<Self> {
        sections
            .into_iter()
            .map(|section| Self::new(api, section))
            .collect()
    }

    pub fn get(&self) -> &Section {
        &self.section
    }

    #[maybe_async::maybe_async]
    pub async fn update(&self, name: &str) -> Result<SectionAPI, TodorstError> {
        self.api.update_section(&self.section.id, name).await
    }

    #[maybe_async::maybe_async]
    pub async fn delete(&self) -> Result<Response, TodorstError> {
        self.api.delete_section(&self.section.id).await
    }

    #[maybe_async::maybe_async]
    pub async fn get_project(&self) -> Result<ProjectAPI, TodorstError> {
        self.api.get_project(&self.section.project_id).await
    }

    #[maybe_async::maybe_async]
    pub async fn get_tasks(&self) -> Result<Vec<TaskAPI>, TodorstError> {
        let query = GetTasksQuery::new().with_section_id(&self.section.id);
        self.api.get_tasks_with_query(query).await
    }

    #[maybe_async::maybe_async]
    pub async fn create_task(&self, body: CreateTaskBody) -> Result<TaskAPI, TodorstError> {
        self.api
            .create_task(body.with_section_id(&self.section.id))
            .await
    }
}
