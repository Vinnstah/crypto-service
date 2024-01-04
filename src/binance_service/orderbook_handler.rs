use axum::extract;
use axum::http::StatusCode;
use reqwest::{self};
use serde::{Deserialize, Serialize};
use std::{env, vec};

use crate::binance_service::binance_client::BinanceClient;

#[derive(Deserialize, Serialize)]
pub struct OrderBookResponse {
    asks: Vec<Vec<String>>,
    bids: Vec<Vec<String>>,

    #[serde(rename = "lastUpdateId")]
    last_update_id: usize,
}

impl OrderBookResponse {
    pub fn new(asks: Vec<Vec<String>>, bids: Vec<Vec<String>>, last_update_id: usize) -> Self {
        Self {
            asks,
            bids,
            last_update_id,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct OrderBookRequest {
    symbol: String,
    limit: Option<u16>,
}




#[axum::debug_handler]
pub async fn get_order_book(
    extract::State(binance_client): extract::State<BinanceClient>,
    axum::Json(payload): axum::Json<OrderBookRequest>,
) -> Result<(axum::http::StatusCode, axum::Json<OrderBookResponse>), axum::http::StatusCode> {
    let client = reqwest::Client::new();


let symbol_param = [("symbol", payload.symbol)];
let mut limit_param: Vec<(&str, u16)> = vec![];
if let Some(limit) = payload.limit {
    limit_param.push(("limit", limit));
    }


    let mut url = binance_client.base_url.clone();
    url.push_str("depth");

    let response = client
        .get(url)
        .headers(binance_client.headers.clone())
        .query(&symbol_param)
        .query(&limit_param)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<OrderBookResponse>().await {
            Ok(order_book) => {
                println!("OK");
                Ok((StatusCode::OK, axum::Json(order_book)))
            }
            Err(err) => {
                println!("{}", err);
                Err(StatusCode::BAD_GATEWAY)
            }
        },
        _other => Err(StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::UNPROCESSABLE_ENTITY)),
    }
}
