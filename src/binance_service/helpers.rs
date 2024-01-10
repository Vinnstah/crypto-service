use std::env::set_current_dir;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct OrderBookResponse {
    pub asks: Vec<Vec<String>>,
    pub bids: Vec<Vec<String>>,

    #[serde(rename = "lastUpdateId")]
    pub last_update_id: usize,
}

impl OrderBookResponse {
    pub fn new() -> Self {
        Self {
            asks: vec![vec![String::new()]],
            bids: vec![vec![String::new()]],
            last_update_id: 0,
        }
    }

    pub fn from(asks: Vec<Vec<String>>, bids: Vec<Vec<String>>, last_update_id: usize) -> Self {
        Self {
            asks,
            bids,
            last_update_id,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct OrderBookRequest {
    pub symbol: Value,
    pub limit: Option<Value>,
}

impl OrderBookRequest {
    pub fn from(symbol: Value, limit: Option<Value>) -> Self {
        Self { symbol, limit }
    }
    pub fn new() -> Self {
        Self {
            symbol: serde_json::Value::String(String::new()),
            limit: None,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct RecentTradesResponse {
    pub id: usize,
    pub price: String,

    #[serde(rename = "qty")]
    pub quantity: String,

    #[serde(rename = "quoteQty")]
    pub quote_quantity: String,
    pub time: usize,

    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: bool,

    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_orderbook_response() {
        let orderbook_response = OrderBookResponse::new();
        assert_eq!(orderbook_response.asks, vec![vec![String::new()]]);
        assert_eq!(orderbook_response.bids, vec![vec![String::new()]]);
        assert_eq!(orderbook_response.last_update_id, 0)
    }

    #[test]
    fn orderbook_response_from_data() {
        let orderbook_response = OrderBookResponse::from(
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
    fn orderbook_response_from_data_assert_data() {
        let orderbook_response = OrderBookResponse::from(
            vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]],
            vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]],
            7010139557,
        );
        assert_eq!(
            orderbook_response.asks,
            vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]]
        );
        assert_eq!(
            orderbook_response.bids,
            vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]]
        );
        assert_eq!(orderbook_response.last_update_id, 7010139557);
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

        let orderbook_response = OrderBookResponse::from(
            vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]],
            vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]],
            7010139557,
        );

        assert_eq!(deserialized_orderbook_response, orderbook_response)
    }

    #[test]
    fn serialize_orderbook_response() {
        let orderbook_response = OrderBookResponse::from(
            vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]],
            vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]],
            7010139557,
        );

        let orderbook_response_json = r#"{"asks":[["0.05161000","32.45550000"]],"bids":[["0.05160000","133.57940000"]],"lastUpdateId":7010139557}"#;

        let serialized_orderbook_response = serde_json::to_string(&orderbook_response).unwrap();

        assert_eq!(serialized_orderbook_response, orderbook_response_json)
    }

    #[test]
    fn orderbook_payload_with_limit() {
        let orderbook_request = r#"{"symbol": "ETHBTC", "limit": 1}"#;

        let payload: OrderBookRequest =
            serde_json::from_str(orderbook_request).expect("Failed to deserialize request");
        assert_eq!(
            payload,
            OrderBookRequest {
                symbol: Value::String("ETHBTC".to_string()),
                limit: Some(Value::Number(1.into()))
            }
        );
    }

    #[test]
    fn orderbook_payload_no_limit() {
        let orderbook_request = r#"{"symbol": "ETHBTC"}"#;

        let payload: OrderBookRequest =
            serde_json::from_str(orderbook_request).expect("Failed to deserialize request");
        assert_eq!(
            payload,
            OrderBookRequest {
                symbol: Value::String("ETHBTC".to_string()),
                limit: None
            }
        );
    }
}
