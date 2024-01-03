use axum::extract;
use axum::http::StatusCode;
use reqwest;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct OrderBookResponse {
    asks: Vec<Vec<(f32, f32)>>,

    bids: Vec<Vec<(f32, f32)>>,
    #[serde(rename = "lastUpdateId")]
    last_update_id: u32,
}

impl OrderBookResponse {
    pub fn new(
        asks: Vec<Vec<(f32, f32)>>,
        bids: Vec<Vec<(f32, f32)>>,
        last_update_id: u32,
    ) -> Self {
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

impl OrderBookRequest {
    fn new(symbol: String, limit: Option<u16>) -> Self {
        Self { symbol, limit }
    }
}

#[derive(Clone, Debug)]
pub struct BinanceClient {
    api_key: String,
    headers: HeaderMap,
    base_url: String,
}

impl BinanceClient {
    /// Creates a new [`BinanceClient`].
    pub fn new() -> Self {
        Self {
            api_key: env::var("BINANCE_API_KEY").expect("No API-key found"),
            headers: {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "X-MBX-APIKEY",
                    env::var("BINANCE_API_KEY")
                        .expect("No API-key found")
                        .parse()
                        .expect("Failed to parse header"),
                );
                headers.insert(
                    "Content-Type",
                    "application/x-www-form-urlencoded"
                        .parse()
                        .expect("Failed to parse header"),
                );
                headers
            },
            base_url: "https://api.binance.com/api/v3/".to_string(),
        }
    }
}

impl Default for BinanceClient {
    fn default() -> Self {
        Self::new()
    }
}
#[axum::debug_handler]
pub async fn get_order_book(
    extract::State(binance_client): extract::State<BinanceClient>,
    axum::Json(payload): axum::Json<OrderBookRequest>,
) -> Result<(axum::http::StatusCode, axum::Json<OrderBookResponse>), axum::http::StatusCode> {
    let client = reqwest::Client::new();

    let params = [("symbol", payload.symbol)];

    let mut url = binance_client.base_url.clone();
    url.push_str("depth");

    let response = client
        .get(&binance_client.base_url)
        .headers(binance_client.headers.clone())
        .form(&params)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<OrderBookResponse>().await {
            Ok(order_book) => Ok((StatusCode::OK, axum::Json(order_book))),
            Err(_) => todo!(),
        },
        _other => Err(StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
