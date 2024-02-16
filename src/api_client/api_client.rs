use core::fmt::Debug;

use uniffi::Lower;

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub http_client: reqwest::Client,
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}


impl ApiClient {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_api_client() {
        let api_client = ApiClient::new();
    }
}
