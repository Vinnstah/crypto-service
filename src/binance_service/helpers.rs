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
    pub symbol: Value,
    pub limit: Value,
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
