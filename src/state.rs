use crate::{
    api_client::api_client::ApiClient, binance_service::binance_client::BinanceClient,
    coinapi_service::coinapi_client::CoinApiClient,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub binance_client: BinanceClient,
    pub coinapi_client: CoinApiClient,
}
