#[derive(Debug, Clone)]
pub struct Config {
    pub user_agent: String,
    pub access_token: Option<String>,
}

impl Config {
    pub fn new(user_agent: String, access_token: Option<String>) -> Config {
        Config {
            user_agent: user_agent,
            access_token: access_token,
        }
    }
}
