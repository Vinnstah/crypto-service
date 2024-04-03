use crate::coin_watch_service::models::Coin;
use crate::coin_watch_service::models::ListOfCoinsRequest;
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use std::collections::HashMap;
use std::convert::identity;
use std::fmt::Debug;
use std::sync::Arc;
use uniffi::{export, Object, Record};
use crate::api_client::gateway::ClientKeys;

use super::error::{
    FFIBridgeError, FFINetworkingError, FFISideError,
    RustSideError,
};

#[derive(PartialEq, Debug, Clone)]
pub struct CoinWatchExternalClient {
    pub headers: HashMap<String, String>,
    pub base_url: String,
}

impl Default for CoinWatchExternalClient {
    fn default() -> Self {
        Self::new("Test".to_string())
    }
}

impl CoinWatchExternalClient {
    pub fn new(key: String) -> Self {
        Self {
            headers: {
                let mut headers: HashMap<String, String> =
                    HashMap::new();
                headers.insert(
                    "x-api-key".to_string(),
                    key, // env::var("LIVE_COIN_WATCH_API_KEY")
                         //     .expect("No API-key found for Coin Watch")
                         //     .parse()
                         //     .expect("Failed to parse header for Coin Watch"),
                );
                headers.insert(
                    "ACCEPT".to_string(),
                    "application/json"
                        .parse()
                        .to_owned()
                        .expect("Failed to parse header for Coin Watch")
                        ,
                );
                headers
            },
            base_url: "https://api.livecoinwatch.com"
                .to_string(),
        }
    }
}

impl ExternalClient for CoinWatchExternalClient {
    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }
}

pub trait ExternalClient: Debug {
    fn get_base_url(&self) -> String;
    fn get_headers(&self) -> HashMap<String, String>;
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NetworkAntenna: Send + Sync {
    async fn make_request(
        &self,
        request: FFINetworkingRequest,
    ) -> Result<FFINetworkingResponse, FFINetworkingError>;
    fn get_binance_key(&self) -> ClientKeys;
    fn get_coinwatch_key(&self) -> ClientKeys;
    fn get_alpha_key(&self) -> ClientKeys;
}

#[derive(Record, Clone, Debug, PartialEq, Eq)]
pub struct FFINetworkingRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,

    pub body: Vec<u8>,
}

#[derive(Record, Clone, Debug, PartialEq, Eq)]
pub struct FFINetworkingResponse {
    pub status_code: u16,

    /// Can be empty.
    pub body: Vec<u8>,
}

impl From<FFINetworkingError> for FFIBridgeError {
    fn from(value: FFINetworkingError) -> Self {
        Self::FromFFI {
            error: value.into(),
        }
    }
}
