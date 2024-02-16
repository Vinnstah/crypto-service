use anyhow::Error;
use axum::{extract::{self, Query, State}};

use crate::{
    coinapi_service::helpers::{AssetIcons, SymbolsResponse},
    state::AppState,
};

use super::helpers::{AssetIconsParams, SymbolsParams};

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

