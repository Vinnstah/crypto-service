use reqwest::header::{HeaderMap, CONTENT_TYPE};
use std::env;

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default_client() {
        env::set_var("BINANCE_API_KEY", "Bearer Key");

        let default_client = BinanceClient::default();
        assert_eq!(default_client, BinanceClient::new())
    }
}
