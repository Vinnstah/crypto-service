use std::env;
use reqwest::header::{CONTENT_TYPE, HeaderMap};

#[derive(Clone, Debug)]
pub struct BinanceClient {
    pub headers: HeaderMap,
    pub base_url: String,
}

impl BinanceClient {
    /// Creates a new [`BinanceClient`].
    pub fn new() -> Self {
        Self {
            headers: {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "X-MBX-APIKEY",
                    env::var("BINANCE_API_KEY")
                        .expect("No API-key found")
                        .parse()
                        .expect("Failed to parse header"),
                );
                headers.insert(
                    CONTENT_TYPE,
                    "application/x-www-form-urlencoded"
                        .parse()
                        .expect("Failed to parse header"),
                );
                headers
            },
            base_url: "https://api.binance.com/api/v3/".to_string(),
        }
    }
}

impl Default for BinanceClient {
    fn default() -> Self {
        Self::new()
    }
}