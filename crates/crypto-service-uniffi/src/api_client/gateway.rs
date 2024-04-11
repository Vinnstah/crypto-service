use crate::{
    api_client::network_antenna::{
        ExternalClient, FFINetworkingRequest,
        FFINetworkingResponse,
    },
    coin_watch_service::models::{
        CoinHistory, CoinHistoryRequest, CoinMeta,
        CoinMetaRequest, ListOfCoinsRequest,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use std::convert::identity;
use std::sync::Arc;
use uniffi::{export, Object, Record};

use super::error::{FFIBridgeError, RustSideError};
use crate::api_client::network_antenna::{
    CoinWatchExternalClient, NetworkAntenna,
};

#[derive(Object)]
pub struct Gateway {
    pub network_antenna: Arc<dyn NetworkAntenna>,
}

#[derive(Record)]
pub struct ClientKeys {
    pub binance: String,
    pub coin_watch: String,
    pub alpha: String,
}

#[export]
impl Gateway {
    /// Constructs a new [`Gateway`] using a "network antenna" - a type
    /// implementing [`NetworkAntenna`] on the FFI side (Swift side), e.g.
    /// `[Swift]URLSession` which wraps the execution of a network call.
    #[uniffi::constructor]
    pub fn new(
        network_antenna: Arc<dyn NetworkAntenna>,
    ) -> Self {
        Self { network_antenna }
    }

    pub async fn get_list_of_coins(
        &self,
        limit: u32,
    ) -> Result<Vec<CoinMeta>, FFIBridgeError> {
        let external_client = CoinWatchExternalClient::new(
            self.network_antenna.get_api_keys().coin_watch,
        );
        let request = ListOfCoinsRequest::new(limit);

        self.post::<_, Vec<CoinMeta>, Vec<CoinMeta>, _, _, _>(
            "/coins/list",
            request,
            res_id,
            external_client,
        )
        .await
    }

    pub async fn get_coin_meta_info(
        &self,
        request: CoinMetaRequest,
    ) -> Result<CoinMeta, FFIBridgeError> {
        let external_client = CoinWatchExternalClient::new(
            self.network_antenna.get_api_keys().coin_watch,
        );

        self.post::<_, CoinMeta, CoinMeta, _, _, _>(
            "/coins/single",
            request,
            res_id,
            external_client,
        )
        .await
    }

    pub async fn get_coin_history_info(
        &self,
        request: CoinHistoryRequest,
    ) -> Result<CoinHistory, FFIBridgeError> {
        let external_client = CoinWatchExternalClient::new(
            self.network_antenna.get_api_keys().coin_watch,
        );

        self.post::<_, CoinHistory, CoinHistory, _, _, _>(
            "/coins/single/history",
            request,
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
        U: for<'a> Deserialize<'a> + std::fmt::Debug,
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
        T: Serialize + std::fmt::Debug,
        U: for<'a> Deserialize<'a> + std::fmt::Debug,
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
            headers: client.get_headers(),
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
        T: Serialize + std::fmt::Debug,
        U: for<'a> Deserialize<'a> + std::fmt::Debug,
        F: Fn(U) -> Result<V, E>,
        E: Into<FFIBridgeError>,
        C: ExternalClient,
    {
        self.make_request::<_, U, V, _, _, _>(
            path, "POST", request, map, client,
        )
        .await
    }
}

const fn res_id<T>(x: T) -> Result<T, FFIBridgeError> {
    identity::<Result<T, FFIBridgeError>>(Ok(x))
}
