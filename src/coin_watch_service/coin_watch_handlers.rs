use super::{coin_watch_client::CoinWatchClient, models::ListOfCoinsRequest};
use crate::{
    coin_watch_service::models::{Coin, CoinMeta, CoinMetaRequest}, state::AppState,
};
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

#[axum::debug_handler]
pub async fn get_list_of_coins(
    State(state): State<AppState>,
    Json(body): Json<ListOfCoinsRequest>,
) -> Result<(StatusCode, Json<Vec<Coin>>), (StatusCode, Json<String>)> {
    state
        .api_client
        .post::<Vec<Coin>, CoinWatchClient, ListOfCoinsRequest>(
            state.coin_watch_client,
            "/coins/list",
            body,
        )
        .await
}

#[axum::debug_handler]
pub async fn get_coin_meta_info(
    State(state): State<AppState>,
    Json(body): Json<CoinMetaRequest>,
) -> Result<(StatusCode, Json<CoinMeta>), (StatusCode, Json<String>)> {
    state
        .api_client
        .post::<CoinMeta, CoinWatchClient, CoinMetaRequest>(
            state.coin_watch_client,
            "/coins/single",
            body,
        )
        .await
}