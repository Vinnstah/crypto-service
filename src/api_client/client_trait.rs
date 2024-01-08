use crate::{
    binance_service::binance_client::BinanceClient, coinapi_service::coinapi_client::CoinApiClient,
};
use core::fmt::Debug;
use reqwest::header::HeaderMap;

pub trait Client: Debug {
    fn get_base_url(&self) -> String;
    fn get_headers(&self) -> HeaderMap;
}

impl Client for BinanceClient {
    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }
    fn get_headers(&self) -> HeaderMap {
        self.headers.clone()
    }
}

impl Client for CoinApiClient {
    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    fn get_headers(&self) -> HeaderMap {
        self.headers.clone()
    }
}
