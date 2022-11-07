pub mod client;
pub mod endpoints;
pub mod errors;
pub mod models;

use client::header::{HeaderMap, HeaderValue};

pub struct Todorst {
    client: client::Client,
    api_token: String,
    user_agent: String,
}

impl Todorst {
    fn create_client(api_token: &str, user_agent: &str) -> client::Client {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Bearer {}", api_token).as_str()).unwrap(),
        );

        headers.insert("User-Agent", HeaderValue::from_str(user_agent).unwrap());

        let client = client::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        client
    }

    pub fn new(api_token: &str) -> Self {
        let user_agent = &format!("todorst/{}", env!("CARGO_PKG_VERSION"));
        let client = Self::create_client(api_token, user_agent);

        Self {
            client,
            api_token: api_token.to_string(),
            user_agent: user_agent.to_string(),
        }
    }

    pub fn new_with_user_agent(api_token: &str, user_agent: &str) -> Self {
        let client = Self::create_client(api_token, user_agent);

        Self {
            client,
            api_token: api_token.to_string(),
            user_agent: user_agent.to_string(),
        }
    }

    pub fn client(&self) -> &client::Client {
        &self.client
    }

    pub fn api_token(&self) -> &str {
        self.api_token.as_str()
    }

    pub fn user_agent(&self) -> &str {
        self.user_agent.as_str()
    }

    #[maybe_async::maybe_async]
    pub async fn get_tasks(&self) -> Result<Vec<models::Task>, errors::TodorstError> {
        let url = endpoints::get_rest_url(endpoints::TASKS_ENDPOINT);
        let response = self.client.get(url).send().await?;
        let tasks: Vec<models::Task> = response.json().await?;
        Ok(tasks)
    }
}
