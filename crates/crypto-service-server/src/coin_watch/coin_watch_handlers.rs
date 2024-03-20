use std::result;

use super::coin_watch_client::CoinWatchClient;
use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use crypto_service::coin_watch_service::models::{
    AggregatedCoinInformation, Coin, CoinMeta, CoinMetaRequest, ListOfCoinsRequest,
};
use serde::Serialize;

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

#[axum::debug_handler]
pub async fn get_aggregated_coin_list(
    State(state): State<AppState>,
    Json(body): Json<ListOfCoinsRequest>,
) -> Result<Json<Vec<AggregatedCoinInformation>>, StatusCode> {
    // let body = ListOfCoinsRequest::new(body);

    let list_of_coins = state
        .api_client
        .post::<Vec<Coin>, CoinWatchClient, ListOfCoinsRequest>(
            state.clone().coin_watch_client,
            "/coins/list",
            body,
        )
        .await
        .expect("Failed to receive result");
    println!("{:#?}", list_of_coins);

    let mut coin_images: Vec<String> = vec![];
    for coin in &list_of_coins.1.0 {
        let coin_body = CoinMetaRequest::new(coin.code.clone());
        println!("{:#?}", coin_body);
        coin_images.push(
            state
                .api_client
                .post::<CoinMeta, CoinWatchClient, CoinMetaRequest>(
                    state.clone().coin_watch_client,
                    "/coins/single",
                    coin_body,
                )
                .await
                .map(|x| x.1.png64.clone())
                .unwrap(),
        );
    }
    println!("{:#?}", coin_images);
    let mut list_of_aggregated_coins: Vec<AggregatedCoinInformation> = vec![];
    for (idx, coin) in list_of_coins.1 .0.iter().enumerate() {
        list_of_aggregated_coins.push(AggregatedCoinInformation {
            name: coin.code.clone(),
            symbol: coin.code.clone(),
            rank: 1.into(),
            rate: coin.rate,
            color: "1".into(),
            png64: coin_images[idx].clone(),
        })
    }
    // let aa = c.into_iter().map(|c| list_of_aggregated_coins.push(AggregatedCoinInformation::new(c.code.clone(), c.code, 1.into(), c.rate)) );
    // println!("{:#?}", list_of_aggregated_coins);
    return Ok(axum::Json(list_of_aggregated_coins));
}
