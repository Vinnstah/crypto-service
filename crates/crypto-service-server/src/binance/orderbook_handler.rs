use std::collections::HashMap;

use crate::{binance::binance_client::BinanceClient, state::AppState};
use axum::extract::{self, Query};
use crypto_service_uniffi::{binance_service::models::{OrderBook, OrderBookRequest, Params, RecentTradesResponse}, client_trait::QueryItems};
use serde::{Deserialize, Serialize};
use serde_json::Value;



#[axum::debug_handler]
pub async fn get_order_book(
    extract::State(state): extract::State<AppState>,
    Query(params): Query<Params>,
) -> Result<
    (axum::http::StatusCode, axum::Json<OrderBook>),
    (axum::http::StatusCode, axum::Json<String>),
> {
    state
        .api_client
        .get::<Params, OrderBook, BinanceClient>(state.binance_client, "depth", params)
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

    //         let result: (axum::http::StatusCode, axum::Json<OrderBook>) =
    //         get_order_book(axum::extract::State(state), axum::Json(payload))
    //         .await
    //         .unwrap();

    //     assert_eq!(result.0, StatusCode::OK);
    //     assert_ne!(
    //         result.1 .0,
    //         OrderBook::new(
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
