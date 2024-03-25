use std::{collections::HashMap, env};

use axum::extract::{self, Query, State};
use crypto_service::{alphavantage_service::models::TopAndBottomTrades, client_trait::QueryItems};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

pub async fn get_top_gainers_and_losers(
    State(state): extract::State<AppState>,
    Query(params): Query<GainersLosersParams>,
) -> Result<
    (axum::http::StatusCode, axum::Json<TopAndBottomTrades>),
    (axum::http::StatusCode, axum::Json<String>),
> {
state
.api_client
.get(state.alpha_client, "", params)
.await

}

#[derive(Debug, Deserialize, Serialize)]
pub struct GainersLosersParams {
    pub function: String,
    pub key: String,
}

impl QueryItems for GainersLosersParams {
    type Query = String;

    fn get_all_queries(&self) -> std::collections::HashMap<&str, Self::Query> {
        let key = env::var("ALPHA_VANTAGE_KEY").expect("Failed to load Alpha key");

        let mut hash = HashMap::new();
        hash.insert("function", "TOP_GAINERS_LOSERS".to_string());
        hash.insert("apikey", key);
        hash
    }
}
// pub async fn get_asset_icons(
//     State(state): extract::State<AppState>,
//     Query(params): Query<AssetIconsParams>,
// ) -> Result<
//     (axum::http::StatusCode, axum::Json<Vec<AssetIcons>>),
//     (axum::http::StatusCode, axum::Json<String>),
// > {
//     state
//         .api_client
//         .get(state.coinapi_client, "assets/icons/", params)
//         .await
// }

// pub async fn get_symbols(
//     State(state): extract::State<AppState>,
//     Query(params): Query<SymbolsParams>,
// ) -> Result<
//     (axum::http::StatusCode, axum::Json<Vec<SymbolsResponse>>),
//     (axum::http::StatusCode, axum::Json<String>),
// > {
//     state
//         .api_client
//         .get(state.coinapi_client, "symbols", params)
//         .await
// }

// #[uniffi::export]
// pub async fn get_symbols_binding(params: SymbolsParams) -> Vec<SymbolsResponse> {
//     let binance_client: BinanceClient = BinanceClient::new();
//     let coinapi_client: CoinApiClient = CoinApiClient::new();
//     let api_client = ApiClient::new();

//     let state = AppState::new(binance_client, coinapi_client, api_client);

//     get_symbols(
//         axum::extract::State(state),
//         Query::from(axum::extract::Query(params)),
//     )
//     .await
//     .map(|x| x.1 .0)
//     .unwrap()
// }
