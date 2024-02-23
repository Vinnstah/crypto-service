use axum::extract::{self, Query, State};
use crypto_service_uniffi::coinapi_service::models::{AssetIcons, AssetIconsParams, SymbolsParams, SymbolsResponse};

use crate::{
    state::AppState
};


pub async fn get_asset_icons(
    State(state): extract::State<AppState>,
    Query(params): Query<AssetIconsParams>,
) -> Result<
    (axum::http::StatusCode, axum::Json<Vec<AssetIcons>>),
    (axum::http::StatusCode, axum::Json<String>),
> {
    state
        .api_client
        .get(state.coinapi_client, "assets/icons/", params)
        .await
}

pub async fn get_symbols(
    State(state): extract::State<AppState>,
    Query(params): Query<SymbolsParams>,
) -> Result<
    (axum::http::StatusCode, axum::Json<Vec<SymbolsResponse>>),
    (axum::http::StatusCode, axum::Json<String>),
> {
    state
        .api_client
        .get(state.coinapi_client, "symbols", params)
        .await
}

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

