use core::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub http_client: reqwest::Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }
}
