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
    pub async fn get_tasks(&self) -> Result<Vec<Task>, TodorstError> {
        let url = rest_tasks_url();
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }
}
