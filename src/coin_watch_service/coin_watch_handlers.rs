use super::{coin_watch_client::CoinWatchClient, models::{ListOfCoinsRequest}};
use crate::{
    api_client::client_trait::EmptyBody, coin_watch_service::models::Coin, state::AppState,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

#[axum::debug_handler]
pub async fn get_list_of_coins(
    State(state): State<AppState>,
    Json(body): Json<ListOfCoinsRequest>,
) -> Result<(StatusCode, Json<Vec<Coin>>), (StatusCode, Json<String>)> {
    println!("{:#?}",body.clone());
    state
        .api_client
        .post::<Vec<Coin>, CoinWatchClient, ListOfCoinsRequest>(
            state.coin_watch_client,
            "/coins/list",
            body,
        )
        .await


}
