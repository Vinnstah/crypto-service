use core::fmt::Debug;
use crypto_service::client_trait::Client;
use reqwest::header::{HeaderMap, ACCEPT};
use std::env;


#[derive(PartialEq, Debug, Clone)]
pub struct CoinWatchClient {
    pub headers: HeaderMap,
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
                let mut headers = HeaderMap::new();
                headers.insert(
                    "x-api-key",
                    env::var("LIVE_COIN_WATCH_API_KEY")
                        .expect("No API-key found for Coin Watch")
                        .parse()
                        .expect("Failed to parse header for Coin Watch"),
                );
                headers.insert(
                    ACCEPT,
                    "application/json"
                        .parse()
                        .expect("Failed to parse header for Coin Watch"),
                );
                headers
            },
            base_url: "https://api.livecoinwatch.com".to_string(),
        }
    }
}

impl Client for CoinWatchClient {
    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    fn get_headers(&self) -> HeaderMap {
        self.headers.clone()
    }
}