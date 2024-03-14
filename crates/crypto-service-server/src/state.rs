use crate::{api_client::api_client::ApiClient, binance::binance_client::BinanceClient, coin_api::coinapi_client::CoinApiClient, coin_watch::coin_watch_client::CoinWatchClient};


#[derive(Debug, Clone)]
pub struct AppState {
    pub binance_client: BinanceClient,
    pub coinapi_client: CoinApiClient,
    pub coin_watch_client: CoinWatchClient,
    pub api_client: ApiClient,
}

impl AppState {
    pub fn new(
        binance_client: BinanceClient,
        coinapi_client: CoinApiClient,
        coin_watch_client: CoinWatchClient,
        api_client: ApiClient,
    ) -> Self {
        Self {
            binance_client,
            coinapi_client,
            coin_watch_client,
            api_client,
        }
    }
}