use axum::extract::{self, Query};

use crate::{
    coinapi_service::helpers::{AssetIcons, SymbolsParams, SymbolsResponse},
    state::AppState,
};

use super::helpers::AssetIconsParams;

#[axum::debug_handler]
pub async fn get_asset_icons(
    extract::State(state): extract::State<AppState>,
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

#[axum::debug_handler]
pub async fn get_symbols(
    extract::State(state): extract::State<AppState>,
    Query(params): Query<AssetIconsParams>,
) -> Result<
    (axum::http::StatusCode, axum::Json<Vec<SymbolsResponse>>),
    (axum::http::StatusCode, axum::Json<String>),
> {
    state
        .api_client
        .get(state.coinapi_client, "symbols", params)
        .await
}
