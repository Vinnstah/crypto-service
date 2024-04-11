use crate::api_client::gateway::ClientKeys;
use std::collections::HashMap;
use std::fmt::Debug;
use uniffi::Record;

use super::error::{FFIBridgeError, FFINetworkingError};

#[derive(PartialEq, Debug, Clone)]
pub struct CoinWatchExternalClient {
    pub headers: HashMap<String, String>,
    pub base_url: String,
}

impl Default for CoinWatchExternalClient {
    fn default() -> Self {
        Self::new("Default Key".to_string())
    }
}

impl CoinWatchExternalClient {
    pub fn new(key: String) -> Self {
        Self {
            headers: {
                let mut headers: HashMap<String, String> =
                    HashMap::new();
                headers
                    .insert("x-api-key".to_string(), key);
                headers.insert(
                    "content-type".to_string(),
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

    fn get_api_keys(&self) -> ClientKeys;
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

#[cfg(test)]
mod tests {
    use crate::api_client::{
        error::{
            FFIBridgeError, FFINetworkingError,
            FFISideError,
        },
        network_antenna::{
            CoinWatchExternalClient, ExternalClient,
        },
    };

    #[test]
    fn equality() {
        assert_eq!(
            CoinWatchExternalClient::new("Link".into()),
            CoinWatchExternalClient::new("Link".into())
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            CoinWatchExternalClient::new("Link".into()),
            CoinWatchExternalClient::new("Zelda".into())
        );
    }

    #[test]
    fn new_coinwatch_client_with_key() {
        assert_eq!(
            CoinWatchExternalClient::new("API_KEY".into())
                .headers
                .get("x-api-key")
                .expect("Failed to unwrap")
                .to_owned(),
            "API_KEY".to_string()
        );
    }

    #[test]
    fn new_coinwatch_client_base_url() {
        assert_eq!(
            CoinWatchExternalClient::new("API_KEY".into())
                .base_url,
            "https://api.livecoinwatch.com".to_string()
        );
    }

    #[test]
    fn new_coinwatch_client_content_type() {
        assert_eq!(
            CoinWatchExternalClient::new("API_KEY".into())
                .headers
                .get("content-type")
                .expect("Failed to unwrap")
                .to_owned(),
            "application/json".to_string()
        );
    }

    #[test]
    fn coinwatch_client_external_client_trait_base_url() {
        assert_eq!(
            CoinWatchExternalClient::new("API_KEY".into())
                .get_base_url(),
            "https://api.livecoinwatch.com".to_string()
        );
    }

    #[test]
    fn coinwatch_client_external_client_trait_key_header() {
        assert_eq!(
            CoinWatchExternalClient::new("API_KEY".into())
                .get_headers()
                .get("x-api-key")
                .expect("Failed to unwrap")
                .to_owned(),
            "API_KEY".to_string()
        );
    }

    #[test]
    fn coinwatch_client_external_client_trait_content_header(
    ) {
        assert_eq!(
            CoinWatchExternalClient::new("API_KEY".into())
                .get_headers()
                .get("content-type")
                .expect("Failed to unwrap")
                .to_owned(),
            "application/json".to_string()
        );
    }

    #[test]
    fn default_impl_for_coinwatch_client() {
        assert_eq!(
            CoinWatchExternalClient::default()
                .headers
                .get("x-api-key")
                .expect("Failed to unwrap")
                .to_owned(),
            "Default Key".to_string()
        );
    }

    #[test]
    fn bridge_error_from_networking_error() {
        let network_error =
            FFINetworkingError::RequestFailed {
                status_code: Some(400),
                url_session_underlying_error: None,
                error_message_from_gateway: Some(
                    "Bad Request".into(),
                ),
            };
        assert_eq!(
            FFIBridgeError::from(network_error.clone()),
            FFIBridgeError::FromFFI {
                error: FFISideError::Networking {
                    error: network_error
                }
            }
        );
    }
}
