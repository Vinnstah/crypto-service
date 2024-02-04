use std::collections::HashMap;

use crate::{
    api_client::get::QueryItems,
    binance_service::{
        binance_client::BinanceClient,
        helpers::{OrderBookRequest, OrderBookResponse, RecentTradesResponse},
    },
    state::AppState,
};
use axum::extract::{self, Query};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Params {
    pub symbol: String,
    pub limit: Option<u16>,
}

impl QueryItems for Params {
    type Query = String;

    fn get_all_queries(&self) -> HashMap<&str, Self::Query> {
        let mut hash = HashMap::new();
        hash.insert("symbol", self.symbol.clone());
        if self.limit.is_some() {
            hash.insert("limit", self.limit.unwrap().to_string());
        }
        hash
    }
}

#[axum::debug_handler]
pub async fn get_order_book(
    extract::State(state): extract::State<AppState>,
    Query(params): Query<Params>,
) -> Result<
    (axum::http::StatusCode, axum::Json<OrderBookResponse>),
    (axum::http::StatusCode, axum::Json<String>),
> {
    state
        .api_client
        .get::<Params, OrderBookResponse, BinanceClient>(state.binance_client, "depth", params)
        .await
}

#[axum::debug_handler]
pub async fn get_recent_trades(
    extract::State(state): extract::State<AppState>,
    Query(params): Query<Params>,
) -> Result<
    (
        axum::http::StatusCode,
        axum::Json<Vec<RecentTradesResponse>>,
    ),
    (axum::http::StatusCode, axum::Json<String>),
> {
    state
        .api_client
        .get::<Params, Vec<RecentTradesResponse>, BinanceClient>(
            state.binance_client,
            "trades",
            params,
        )
        .await
}

impl QueryItems for OrderBookRequest {
    type Query = Value;

    fn get_all_queries(&self) -> std::collections::HashMap<&str, Self::Query> {
        let mut hash_map: HashMap<&str, Value> = HashMap::new();
        if Value::is_string(&self.symbol) {
            hash_map.insert("symbol", self.symbol.clone());
        }

        match &self.limit {
            Some(Value::Number(limit)) => {
                hash_map.insert("limit", serde_json::Value::Number(limit.clone()))
            }
            None => None,
            _other => panic!("Failed"),
        };
        hash_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binance_client_and_url() {
        std::env::set_var("BINANCE_API_KEY", "Bearer Key");

        let binance_client = BinanceClient::new();
        let mut url = binance_client.base_url.clone();
        url.push_str("depth");

        assert_eq!(url, "https://api.binance.com/api/v3/depth")
    }

    //     #[tokio::test]
    //     async fn get_orderbook_pass() {
    //         std::env::set_var("BINANCE_API_KEY", "Bearer Key");
    //         std::env::set_var("COINAPI_API_KEY", "Bearer Key");

    //         let mut binance_client = BinanceClient::new();
    //         let coinapi_client = CoinApiClient::new();

    //         binance_client.base_url = "https://api.binance.us/api/v1/".to_string();
    //         let state = AppState {
    //             binance_client,
    //             coinapi_client,
    //         };

    //         let orderbook_request = r#"{"symbol": "ETHBTC", "limit": 1}"#;
    //         let payload: OrderBookRequest = serde_json::from_str(orderbook_request).unwrap();

    //         let result: (axum::http::StatusCode, axum::Json<OrderBookResponse>) =
    //         get_order_book(axum::extract::State(state), axum::Json(payload))
    //         .await
    //         .unwrap();

    //     assert_eq!(result.0, StatusCode::OK);
    //     assert_ne!(
    //         result.1 .0,
    //         OrderBookResponse::new(
    //             vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]],
    //             vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]],
    //             7010139557,
    //         )
    //     );
    // }

    // #[tokio::test]
    // async fn get_orderbook_fail_invalid_symbol() {
    //     std::env::set_var("BINANCE_API_KEY", "Bearer Key");
    //     std::env::set_var("COINAPI_API_KEY", "Bearer Key");

    //     let mut binance_client = BinanceClient::new();
    //     let coinapi_client = CoinApiClient::new();
    //     binance_client.base_url = "https://api.binance.us/api/v1/".to_string();
    //     let state = AppState {
    //         binance_client,
    //         coinapi_client,
    //         };

    //         let orderbook_request = r#"{"symbol": "APA", "limit": 1}"#;
    //         let payload: OrderBookRequest = serde_json::from_str(orderbook_request).unwrap();

    //         let result = get_order_book(axum::extract::State(state), axum::Json(payload)).await;

    //         assert_eq!(result.err().unwrap().0, StatusCode::BAD_REQUEST)
    //     }
}
