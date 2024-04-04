use super::coin_watch_client::CoinWatchClient;
use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use crypto_service::coin_watch_service::models::{
    AggregatedCoinInformation, Coin, CoinHistoryRequest,
    CoinMeta, CoinMetaRequest, ListOfCoinsRequest,
};

pub async fn get_list_of_coins(
    State(state): State<AppState>,
    Json(body): Json<ListOfCoinsRequest>,
) -> Result<
    (StatusCode, Json<Vec<Coin>>),
    (StatusCode, Json<String>),
> {
    state
        .api_client
        .post::<Vec<Coin>, CoinWatchClient, ListOfCoinsRequest>(
            state.coin_watch_client,
            "/coins/list",
            body,
        )
        .await
}

pub async fn get_coin_meta_info(
    State(state): State<AppState>,
    Json(body): Json<CoinMetaRequest>,
) -> Result<
    (StatusCode, Json<CoinMeta>),
    (StatusCode, Json<String>),
> {
    state
        .api_client
        .post::<CoinMeta, CoinWatchClient, CoinMetaRequest>(
            state.coin_watch_client,
            "/coins/single",
            body,
        )
        .await
}

pub async fn get_coin_history_info(
    State(state): State<AppState>,
    Json(body): Json<CoinHistoryRequest>,
) -> Result<
    (StatusCode, Json<CoinMeta>),
    (StatusCode, Json<String>),
> {
    state
        .api_client
        .post::<CoinMeta, CoinWatchClient, CoinHistoryRequest>(
            state.coin_watch_client,
            "/coins/single/history",
            body,
        )
        .await
}

pub async fn get_aggregated_coin_list(
    State(state): State<AppState>,
    Json(body): Json<ListOfCoinsRequest>,
) -> Result<Json<Vec<AggregatedCoinInformation>>, StatusCode>
{
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

    let mut coin_meta: Vec<CoinMeta> = vec![];
    for coin in &list_of_coins.1 .0 {
        let coin_body = CoinMetaRequest::new(
            coin.code.clone().unwrap(),
        );
        coin_meta.push(
            state
                .api_client
                .post::<CoinMeta, CoinWatchClient, CoinMetaRequest>(
                    state.clone().coin_watch_client,
                    "/coins/single",
                    coin_body,
                )
                .await
                .map(|x| x.1 .0)
                .unwrap(),
        );
    }
    let mut list_of_aggregated_coins: Vec<
        AggregatedCoinInformation,
    > = vec![];
    for (idx, coin) in list_of_coins.1 .0.iter().enumerate()
    {
        list_of_aggregated_coins.push(
            AggregatedCoinInformation {
                name: coin_meta[idx].name.clone().unwrap_or("".into()),
                symbol: coin_meta[idx]
                    .symbol
                    .clone()
                    .unwrap_or("0".to_string()),
                rank: coin_meta[idx].rank.unwrap_or(0.into()),
                rate: coin.rate.unwrap(),
                color: coin_meta[idx].color.clone().unwrap_or("".into()),
                png64: coin_meta[idx].png64.clone().unwrap_or("".into()),
            },
        )
    }
    Ok(axum::Json(list_of_aggregated_coins))
}
