use crate::{
    binance_service::binance_client::BinanceClient, coinapi_service::coinapi_client::CoinApiClient,
};
use core::fmt::Debug;
use reqwest::header::HeaderMap;

pub trait Client: Debug {
    fn get_base_url(&self) -> String;
    fn get_headers(&self) -> HeaderMap;
}

impl Client for BinanceClient {
    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }
    fn get_headers(&self) -> HeaderMap {
        self.headers.clone()
    }
}

impl Client for CoinApiClient {
    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    fn get_headers(&self) -> HeaderMap {
        self.headers.clone()
    }
}

#[cfg(test)]
#[derive(Debug)]
struct TestClient {
    base_url: String,
    headers: HeaderMap,
}

#[cfg(test)]
impl Client for TestClient {
    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    fn get_headers(&self) -> HeaderMap {
        self.headers.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn get_base_url() {
        let test_client = TestClient {
            base_url: "http://www.apa.se".to_string(),
            headers: HeaderMap::new(),
        };
        assert_eq!(
            test_client.get_base_url(),
            String::from_str("http://www.apa.se").unwrap()
        )
    }

    #[test]
    fn get_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("apa", "banan".parse().unwrap());

        let test_client = TestClient {
            base_url: String::new(),
            headers: headers,
        };

        assert!(test_client.get_headers().contains_key("apa"));
        assert_eq!(test_client.get_headers().get("apa").unwrap(), "banan")
    }

    #[test]
    fn get_base_url_binance_client() {
        std::env::set_var("BINANCE_API_KEY", "TEST");

        let binance_client = BinanceClient::new();
        assert_eq!(
            binance_client.get_base_url(),
            String::from_str("https://api.binance.com/api/v3/").unwrap()
        )
    }

    #[test]
    fn get_headers_binance_client() {
        std::env::set_var("BINANCE_API_KEY", "TEST");

        let binance_client = BinanceClient::new();

        assert!(binance_client.get_headers().contains_key("X-MBX-APIKEY"));
        assert!(binance_client.get_headers().contains_key("Content-Type"));
        assert_eq!(
            binance_client.get_headers().get("Content-Type").unwrap(),
            "application/x-www-form-urlencoded"
        );
        assert_eq!(
            binance_client.get_headers().get("X-MBX-APIKEY").unwrap(),
            "TEST"
        )
    }
}
