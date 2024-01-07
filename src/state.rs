use crate::{
    binance_service::binance_client::BinanceClient, coinapi_service::coinapi_client::CoinApiClient, api_client::api_client::ApiClient,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub binance_client: BinanceClient,
    pub coinapi_client: CoinApiClient,
}
