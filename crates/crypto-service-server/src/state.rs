use crate::{api_client::api_client::ApiClient, binance::binance_client::BinanceClient, coin_api::coinapi_client::CoinApiClient};


#[derive(Debug, Clone)]
pub struct AppState {
    pub binance_client: BinanceClient,
    pub coinapi_client: CoinApiClient,
    pub api_client: ApiClient,
}

impl AppState {
    pub fn new(
        binance_client: BinanceClient,
        coinapi_client: CoinApiClient,
        api_client: ApiClient,
    ) -> Self {
        Self {
            binance_client,
            coinapi_client,
            api_client,
        }
    }
}
