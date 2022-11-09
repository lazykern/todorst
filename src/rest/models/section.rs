use serde::{Deserialize, Serialize};

use crate::{
    error::TodorstError,
    rest::{request::query::GetTasksQuery, TodorstRestAPI},
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

    pub fn get(&self) -> &Section {
        &self.section
    }

    #[maybe_async::maybe_async]
    pub async fn update(&self, name: &str) -> Result<SectionAPI, TodorstError> {
        self.api.update_section(&self.section.id, name).await
    }

    #[maybe_async::maybe_async]
    pub async fn delete(&self) -> Result<(), TodorstError> {
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
}
