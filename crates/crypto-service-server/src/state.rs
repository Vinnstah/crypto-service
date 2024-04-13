use crate::{
    alphavantage_api::alpha_client::AlphaAdvantageClient,
    api_client::api_client::ApiClient,
    coin_watch::coin_watch_client::CoinWatchClient,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub alpha_client: AlphaAdvantageClient,
    pub coin_watch_client: CoinWatchClient,
    pub api_client: ApiClient,
}

impl AppState {
    pub fn new(
        alpha_client: AlphaAdvantageClient,
        coin_watch_client: CoinWatchClient,
        api_client: ApiClient,
    ) -> Self {
        Self {
            alpha_client,
            coin_watch_client,
            api_client,
        }
    }
}
