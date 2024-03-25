use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client_trait::QueryItems;

#[derive(Deserialize, Serialize, PartialEq, Debug, uniffi::Record)]
pub struct OrderBook {
    pub asks: Vec<Vec<String>>,
    pub bids: Vec<Vec<String>>,

    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            asks: vec![vec![String::new()]],
            bids: vec![vec![String::new()]],
            last_update_id: 0,
        }
    }

    pub fn from(asks: Vec<Vec<String>>, bids: Vec<Vec<String>>, last_update_id: u64) -> Self {
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

impl Default for OrderBookRequest {
    fn default() -> Self {
        Self::new()
    }
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

#[derive(Deserialize, Serialize, PartialEq, Debug, uniffi::Record)]
pub struct RecentTradesResponse {
    pub id: u64,
    pub price: String,

    #[serde(rename = "qty")]
    pub quantity: String,

    #[serde(rename = "quoteQty")]
    pub quote_quantity: String,
    pub time: u64,

    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: bool,

    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
}

#[derive(Debug, Deserialize, Serialize, uniffi::Record)]
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

    // #[test]
    // fn new_orderbook_response() {
    //     let orderbook_response = OrderBook::new();
    //     assert_eq!(orderbook_response.asks, vec![vec![String::new()]]);
    //     assert_eq!(orderbook_response.bids, vec![vec![String::new()]]);
    //     assert_eq!(orderbook_response.last_update_id, 0)
    // }

    // #[test]
    // fn orderbook_response_from_data() {
    //     let orderbook_response = OrderBook::from(
    //         vec![vec!["55".to_string()]],
    //         vec![vec!["44".to_string()]],
    //         12345,
    //     );
    //     assert_eq!(
    //         orderbook_response,
    //         OrderBook {
    //             asks: vec![vec!["55".to_string()]],
    //             bids: vec![vec!["44".to_string()]],
    //             last_update_id: 12345
    //         }
    //     )
    // }

    // #[test]
    // fn orderbook_response_from_data_assert_data() {
    //     let orderbook_response = OrderBook::from(
    //         vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]],
    //         vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]],
    //         7010139557,
    //     );
    //     assert_eq!(
    //         orderbook_response.asks,
    //         vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]]
    //     );
    //     assert_eq!(
    //         orderbook_response.bids,
    //         vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]]
    //     );
    //     assert_eq!(orderbook_response.last_update_id, 7010139557);
    // }

    // #[test]
    // fn deserialize_orderbook_response() {
    //     let orderbook_response_json = r#"
    // {
    //     "asks":[["0.05161000","32.45550000"]],
    //     "bids":[["0.05160000","133.57940000"]],
    //     "lastUpdateId":7010139557}
    // "#;

    //     let deserialized_orderbook_response: OrderBook =
    //         serde_json::from_str(orderbook_response_json).unwrap();

    //     let orderbook_response = OrderBook::from(
    //         vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]],
    //         vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]],
    //         7010139557,
    //     );

    //     assert_eq!(deserialized_orderbook_response, orderbook_response)
    // }

    // #[test]
    // fn serialize_orderbook_response() {
    //     let orderbook_response = OrderBook::from(
    //         vec![vec!["0.05161000".to_string(), "32.45550000".to_string()]],
    //         vec![vec!["0.05160000".to_string(), "133.57940000".to_string()]],
    //         7010139557,
    //     );

    //     let orderbook_response_json = r#"{"asks":[["0.05161000","32.45550000"]],"bids":[["0.05160000","133.57940000"]],"lastUpdateId":7010139557}"#;

    //     let serialized_orderbook_response = serde_json::to_string(&orderbook_response).unwrap();

    //     assert_eq!(serialized_orderbook_response, orderbook_response_json)
    // }

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
