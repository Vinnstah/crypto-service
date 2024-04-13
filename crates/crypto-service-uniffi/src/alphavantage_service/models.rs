use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record, PartialEq)]
pub struct TopAndBottomTrades {
    pub metadata: String,
    pub last_updated: String,
    pub top_gainers: Vec<MostActivelyTraded>,
    pub top_losers: Vec<MostActivelyTraded>,
    pub most_actively_traded: Vec<MostActivelyTraded>,
}

#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record, PartialEq)]
pub struct MostActivelyTraded {
    pub ticker: String,
    pub price: String,
    pub change_amount: String,
    pub change_percentage: String,
    pub volume: String,
}