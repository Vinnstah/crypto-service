use crate::coin_watch_service::models::{
    AggregatedCoinInformation, ListOfCoinsRequest,
};
use reqwest::header::{HeaderMap, ACCEPT};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use std::convert::identity;
use std::env;
use std::fmt::Debug;
use std::sync::Arc;
use std::{collections::HashMap, sync::Mutex};
use tokio::sync::oneshot::{channel, Sender};
use uniffi::{export, Enum, Object, Record};

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
                let mut headers: HashMap<String, String> = HashMap::new();
                headers.insert(
                    "x-api-key".to_string(),
                    key
                    // env::var("LIVE_COIN_WATCH_API_KEY")
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
}

#[derive(Record, Clone, Debug, PartialEq, Eq)]
pub struct FFINetworkingRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,

    pub body: Vec<u8>,
}

const fn res_id<T>(x: T) -> Result<T, FFIBridgeError> {
    identity::<Result<T, FFIBridgeError>>(Ok(x))
}

#[derive(Object)]
pub struct Gateway {
    pub network_antenna: Arc<dyn NetworkAntenna>,
}

#[export]
impl Gateway {
    /// Constructs a new [`GatewayExternalClient`] using a "network antenna" - a type
    /// implementing [`FFIOperationExecutor`] on the FFI side (Swift side), e.g.
    /// `[Swift]URLSession` which wraps the execution of a network call.
    #[uniffi::constructor]
    pub fn new(
        network_antenna: Arc<dyn NetworkAntenna>,
    ) -> Self {
        Self { network_antenna }
    }

    pub async fn get_list_of_agg_coins(
        &self,
        key: String
    ) -> Result<
        Vec<AggregatedCoinInformation>,
        FFIBridgeError,
    > {
        let external_client = CoinWatchExternalClient::new(key);

        self.post(
            "/coins/list",
            ListOfCoinsRequest::new(1),
            res_id,
            external_client,
        )
        .await
    }
}

impl Gateway {
    fn model_from_response<U>(
        &self,
        response: FFINetworkingResponse,
    ) -> Result<U, RustSideError>
    where
        U: for<'a> Deserialize<'a>,
    {
        if let 200..=299 = response.status_code {
            // all good
        } else {
            return Err(RustSideError::BadResponseCode);
        }

        let body = response.body;
        if body.is_empty() {
            return Err(RustSideError::ResponseBodyWasNil);
        }

        serde_json::from_slice::<U>(&body).map_err(|_| {
            RustSideError::UnableJSONDeserializeHTTPResponseBodyIntoTypeName {
                type_name: std::any::type_name::<U>().to_owned(),
            }
        })
    }

    async fn make_request<T, U, V, F, E, C>(
        &self,
        path: &str,
        method: &str,
        request: T,
        map: F,
        client: C,
    ) -> Result<V, FFIBridgeError>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
        F: Fn(U) -> Result<V, E>,
        E: Into<FFIBridgeError>,
        C: ExternalClient,
    {
        // JSON serialize request into body bytes
        let body = to_vec(&request).unwrap();

        // Append relative path to base url
        let url = format!(
            "{}{}",
            client.get_base_url(),
            path.to_owned()
        );

        // Create Network request object, which will be translated by
        // Swift side into a `[Swift]URLRequest`
        let request = FFINetworkingRequest {
            url,
            body,
            method: method.to_owned(),
            headers: client.get_headers(), // HashMap::<String, String>::from_iter(
                                                   //     [(
                                                   //         "Content-Type".to_owned(),
                                                   //         "application/json".to_owned(),
                                                   //     )],
                                                   // ),
        };

        // Let Swift side make network request and await response
        // let response = self.networking_dispatcher.dispatch(request).await?;
        let response = self
            .network_antenna
            .make_request(request)
            .await?;
        // Read out HTTP body from response and JSON parse it into U
        let model =
            self.model_from_response(response).map_err(
                |error| FFIBridgeError::FromRust { error },
            )?;
        // return model
        // Map U -> V
        map(model).map_err(|e| e.into())
    }

    pub(crate) async fn post<T, U, V, F, E, C>(
        &self,
        path: &str,
        request: T,
        map: F,
        client: C,
    ) -> Result<V, FFIBridgeError>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
        F: Fn(U) -> Result<V, E>,
        E: Into<FFIBridgeError>,
        C: ExternalClient,
    {
        self.make_request(
            path,
            "POST",
            request,
            map,
            client,
        )
        .await
    }
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
