use core::fmt::Debug;
use std::{collections::HashMap, env};

use crate::client_trait::Client;

#[derive(PartialEq, Debug, Clone)]
pub struct CoinWatchClient {
    pub headers: HashMap<String, String>,
    pub base_url: String,
}

impl Default for CoinWatchClient {
    fn default() -> Self {
        Self::new()
    }
}

impl CoinWatchClient {
    pub fn new() -> Self {
        Self {
            headers: {
                let mut headers = HashMap::new();
                headers.insert(
                    "x-api-key".to_string(),
                    env::var("LIVE_COIN_WATCH_API_KEY")
                        .expect("No API-key found for Coin Watch")
                        .parse()
                        .expect("Failed to parse header for Coin Watch"),
                );
                headers.insert(
                    "content-type".to_string(),
                    "application/json"
                        .parse()
                        .expect("Failed to parse header for Coin Watch"),
                );
                headers
            },
            base_url: "https://api.livecoinwatch.com"
                .to_string(),
        }
    }
    pub fn new_with_key(key: String) -> Self {
        Self {
            headers: {
                let mut headers = HashMap::new();
                headers
                    .insert("x-api-key".to_string(), key);
                headers.insert(
                    "content-type".to_string(),
                    "application/json"
                        .parse()
                        .expect("Failed to parse header for Coin Watch"),
                );
                headers
            },
            base_url: "https://api.livecoinwatch.com"
                .to_string(),
        }
    }
}

impl Client for CoinWatchClient {
    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }
}
