use crate::binance_service::binance_client::BinanceClient;
use axum::extract;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct OrderBookResponse {
    asks: Vec<Vec<String>>,
    bids: Vec<Vec<String>>,

    #[serde(rename = "lastUpdateId")]
    last_update_id: usize,
}

impl OrderBookResponse {
    pub fn new(asks: Vec<Vec<String>>, bids: Vec<Vec<String>>, last_update_id: usize) -> Self {
        Self {
            asks,
            bids,
            last_update_id,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct OrderBookRequest {
    pub symbol: String,
    pub limit: Option<u16>,
}

#[axum::debug_handler]
pub async fn get_order_book(
    extract::State(binance_client): extract::State<BinanceClient>,
    axum::Json(payload): axum::Json<OrderBookRequest>,
) -> Result<
    (axum::http::StatusCode, axum::Json<OrderBookResponse>),
    (axum::http::StatusCode, axum::Json<String>),
> {
    BinanceClient::get_order_book(binance_client, payload, "depth").await
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn new_orderbook_response() {
        let orderbook_response = OrderBookResponse::new(
            vec![vec!["55".to_string()]],
            vec![vec!["44".to_string()]],
            12345,
        );
        assert_eq!(
            orderbook_response,
            OrderBookResponse {
                asks: vec![vec!["55".to_string()]],
                bids: vec![vec!["44".to_string()]],
                last_update_id: 12345
            }
        )
    }

    #[test]
    fn deserialize_orderbook_response() {
        let orderbook_response_json = r#"
        {
            "asks":[["0.05161000","32.45550000"]],
            "bids":[["0.05160000","133.57940000"]],
            "lastUpdateId":7010139557}
        "#;

        let deserialized_orderbook_response: OrderBookResponse =
            serde_json::from_str(orderbook_response_json).unwrap();

        let orderbook_response = OrderBookResponse::new(
            vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]],
            vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]],
            7010139557,
        );

        assert_eq!(deserialized_orderbook_response, orderbook_response)
    }

    #[test]
    fn serialize_orderbook_response() {
        let orderbook_response = OrderBookResponse::new(
            vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]],
            vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]],
            7010139557,
        );

        let orderbook_response_json = r#"{"asks":[["0.05161000","32.45550000"]],"bids":[["0.05160000","133.57940000"]],"lastUpdateId":7010139557}"#;

        let serialized_orderbook_response = serde_json::to_string(&orderbook_response).unwrap();

        assert_eq!(serialized_orderbook_response, orderbook_response_json)
    }

    #[test]
    fn orderbook_payload_and_params() {
        let orderbook_request = r#"{"symbol": "ETHBTC", "limit": 1}"#;

        let payload: OrderBookRequest =
            serde_json::from_str(orderbook_request).expect("Failed to deserialize request");
        assert_eq!(
            payload,
            OrderBookRequest {
                symbol: "ETHBTC".to_string(),
                limit: Some(1)
            }
        );

        let symbol_param = [("symbol", payload.symbol)];
        assert_eq!(symbol_param, [("symbol", "ETHBTC".to_string())]);

        let mut limit_param: Vec<(&str, u16)> = vec![];
        if let Some(limit) = payload.limit {
            limit_param.push(("limit", limit));
        }
        assert_eq!(limit_param, vec![("limit", 1)])
    }

    #[test]
    fn binance_client_and_url() {
        std::env::set_var("BINANCE_API_KEY", "Bearer Key");

        let binance_client = BinanceClient::new();
        let mut url = binance_client.base_url.clone();
        url.push_str("depth");

        assert_eq!(url, "https://api.binance.com/api/v3/depth")
    }

    #[tokio::test]
    #[cfg(not(tarpaulin))]
    async fn get_orderbook_pass() {
        std::env::set_var("BINANCE_API_KEY", "Bearer Key");
        let binance_client = BinanceClient::new();

        let orderbook_request = r#"{"symbol": "ETHBTC", "limit": 1}"#;
        let payload: OrderBookRequest = serde_json::from_str(orderbook_request).unwrap();

        let result = get_order_book(axum::extract::State(binance_client), axum::Json(payload))
            .await
            .unwrap();

        assert_eq!(result.0, StatusCode::OK);
        assert_ne!(
            result.1 .0,
            OrderBookResponse::new(
                vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]],
                vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]],
                7010139557,
            )
        );
    }

    #[tokio::test]
    #[cfg(not(tarpaulin))]
    async fn get_orderbook_fail_invalid_symbol() {
        std::env::set_var("BINANCE_API_KEY", "Bearer Key");
        let binance_client = BinanceClient::new();

        let orderbook_request = r#"{"symbol": "APA", "limit": 1}"#;
        let payload: OrderBookRequest = serde_json::from_str(orderbook_request).unwrap();

        let result =
            get_order_book(axum::extract::State(binance_client), axum::Json(payload)).await;

        assert_eq!(result.err().unwrap().0, StatusCode::BAD_REQUEST)
    }
}
