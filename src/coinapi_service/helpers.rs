use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api_client::get::QueryItems;

// pub trait CoinAPIQueryItems {
//     type Query;
//     fn get_all_parameters(&self) -> HashMap<&str, Self::Query>;
// }

pub trait CoinAPIResponse {
    type Response;

    fn response_body(&self) -> axum::Json<Self::Response>;
}

#[derive(Serialize, Deserialize, Debug, Default, uniffi::Record)]
pub struct AssetIcons {
    exchange_id: Option<String>,
    asset_id: String,
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, uniffi::Record)]
pub struct SymbolsRequest {}

#[derive(PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct SymbolsResponse {
    symbol_id: Option<String>,
    exchange_id: Option<String>,
    symbol_type: Option<String>,
    asset_id_base: Option<String>,
    asset_id_quote: Option<String>,
    data_start: Option<String>,
    data_end: Option<String>,
    data_quote_start: Option<String>,
    data_quote_end: Option<String>,
    data_orderbook_start: Option<String>,
    data_orderbook_end: Option<String>,
    data_trade_start: Option<String>,
    data_trade_end: Option<String>,
    #[serde(rename = "volume_1hrs")]
    volume_1_hrs: Option<f64>,
    #[serde(rename = "volume_1hrs_usd")]
    volume_1_hrs_usd: Option<f64>,
    #[serde(rename = "volume_1day")]
    volume_1_day: Option<f64>,
    #[serde(rename = "volume_1day_usd")]
    volume_1_day_usd: Option<f64>,
    #[serde(rename = "volume_1mth")]
    volume_1_mth: Option<f64>,
    #[serde(rename = "volume_1mth_usd")]
    volume_1_mth_usd: Option<f64>,
    price: Option<f64>,
    symbol_id_exchange: Option<String>,
    asset_id_base_exchange: Option<String>,
    asset_id_quote_exchange: Option<String>,
    price_precision: Option<f64>,
    size_precision: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, uniffi::Record)]
pub struct AssetIconsRequest {
    pub size: i32,
}

impl QueryItems for AssetIconsRequest {
    type Query = Self;

    fn get_all_queries(&self) -> HashMap<&str, Self::Query> {
        HashMap::new()
    }
}

impl QueryItems for SymbolsRequest {
    type Query = Self;

    fn get_all_queries(&self) -> HashMap<&str, Self::Query> {
        HashMap::new()
    }
}
