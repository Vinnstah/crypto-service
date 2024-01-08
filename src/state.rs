use crate::{
    api_client::api_client::ApiClient, binance_service::binance_client::BinanceClient,
    coinapi_service::coinapi_client::CoinApiClient,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub binance_client: BinanceClient,
    pub coinapi_client: CoinApiClient,
    pub http_client: reqwest::Client
}

impl AppState {
    pub fn new(binance_client: BinanceClient, coinapi_client: CoinApiClient, http_client: reqwest::Client) -> Self { Self { binance_client, coinapi_client, http_client } }
}
