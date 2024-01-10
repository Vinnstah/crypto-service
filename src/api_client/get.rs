use super::{api_client::ApiClient, client_trait::Client};
use axum::http::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

impl ApiClient {
    pub async fn get<T, U, C: Client>(
        &self,
        client_source: C,
        path: &str,
        payload: T,
    ) -> Result<(StatusCode, axum::Json<U>), (StatusCode, axum::Json<String>)>
    where
        <T as QueryItems>::Query: Serialize,
        T: QueryItems + std::fmt::Debug,
        U: DeserializeOwned,
    {
        let mut url = client_source.get_base_url();
        url.push_str(path);

        // Split out construction of a request
        let request = self
            .http_client
            .get(url)
            .headers(client_source.get_headers())
            .query(&payload.get_all_queries())
            .build()
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())))?;

        // Implement Into for API-error type
        let response_bytes = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, axum::Json(e.to_string())))?;

        response_bytes
            .json::<U>()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())))
            .map(|r| (StatusCode::OK, axum::Json::<U>(r)))
    }
}

pub trait QueryItems {
    type Query;
    fn get_all_queries(&self) -> HashMap<&str, Self::Query>;
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_json::Number;

    use crate::{
        api_client,
        binance_service::{
            binance_client::{self, BinanceClient},
            helpers::{OrderBookRequest, OrderBookResponse},
        },
    };

    use super::*;

    #[test]
    fn get_orderbook() {
        std::env::set_var("BINANCE_API_KEY", "TEST");

        let api_client = ApiClient::new();
        let binance_client = BinanceClient::new();
        let orderbook_request = OrderBookRequest {
            symbol: serde_json::Value::String("ETHBTC".to_string()),
            limit: Some(serde_json::Value::Number(Number::from_str("10").unwrap())),
        };

        let response = api_client
            .get::<OrderBookRequest, OrderBookResponse, binance_client::BinanceClient>(
                binance_client,
                "depth",
                orderbook_request,
            );
    }
}
