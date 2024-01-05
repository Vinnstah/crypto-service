use axum::http::StatusCode;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use std::env;

use super::query_trait::QueryItem;

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

    pub async fn get_order_book<Request: QueryItem, Response: DeserializeOwned>(
        binance_client: BinanceClient,
        payload: Request,
        path: &str,
    ) -> Result<
        (axum::http::StatusCode, axum::Json<Response>),
        (axum::http::StatusCode, axum::Json<String>),
    > {
        let client = reqwest::Client::new();

        let symbol_param = [("symbol", payload.get_symbol())];
        let mut limit_param: Vec<(&str, u16)> = vec![];
        if let Some(limit) = payload.get_limit() {
            limit_param.push(("limit", limit));
        }

        let mut url = binance_client.base_url.clone();
        url.push_str(&path);

        let response = client
            .get(url)
            .headers(binance_client.headers.clone())
            .query(&symbol_param)
            .query(&limit_param)
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<Response>().await {
                Ok(order_book) => Ok((StatusCode::OK, axum::Json(order_book))),
                Err(err) => {
                    println!("{}", err);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        axum::Json(err.to_string()),
                    ))
                }
            },
            _other => Err((
                StatusCode::from_u16(response.status().as_u16()).unwrap(),
                axum::Json(response.text().await.unwrap()),
            )),
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
