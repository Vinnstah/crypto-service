use crate::coin_watch_service::coin_watch_client::CoinWatchClient;
use crate::coinapi_service::coinapi_client::CoinApiClient;
use crate::{api_client::api_client::ApiClient, binance_service::binance_client::BinanceClient};

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
