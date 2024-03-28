use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use std::{collections::HashMap, sync::Mutex};
use thiserror::Error as ThisError;
use tokio::sync::oneshot::{channel, Sender};
use uniffi::{export, Enum, Error, Object, Record};

use crate::coin_watch_service::models::{AggregatedCoinInformation, ListOfCoinsRequest};

use super::error::{FFIBridgeError, FFINetworkingError, FFISideError, RustSideError};
// use super::error::{FFIBridgeError, RustSideError};

#[derive(Object)]
pub struct Gateway {
    pub networking_dispatcher: FFIOperationDispatcher<FFINetworkingOutcomeListener>,
}

#[export]
impl Gateway {
    /// Constructs a new [`GatewayClient`] using a "network antenna" - a type
    /// implementing [`FFIOperationExecutor`] on the FFI side (Swift side), e.g.
    /// `[Swift]URLSession` which wraps the execution of a network call.
    #[uniffi::constructor]
    pub fn new(network_antenna: Arc<dyn FFINetworkingExecutor>) -> Self {
        Self {
            networking_dispatcher: FFIOperationDispatcher::<FFINetworkingOutcomeListener>::new(
                network_antenna,
            ),
        }
    }

    pub async fn get_list_of_agg_coins(
        &self,
        address: String,
    ) -> Result<Vec<AggregatedCoinInformation>, FFIBridgeError> {
        self.post(
            "coins/list",
            ListOfCoinsRequest::new(1),
            no_map
        )
        .await
    }
}


fn no_map(item: Vec<AggregatedCoinInformation>) -> Result<Vec<AggregatedCoinInformation>, FFIBridgeError> {
    Ok(item).map_err(|x: FFIBridgeError| FFIBridgeError::FromRust { error: self::RustSideError::BadResponseCode })
}
impl Gateway {

    fn model_from_response<U>(&self, response: FFINetworkingResponse) -> Result<U, RustSideError>
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
            return Err(RustSideError::ResponseBodyWasNil.into());
        }

        serde_json::from_slice::<U>(&body).map_err(|_| {
            RustSideError::UnableJSONDeserializeHTTPResponseBodyIntoTypeName {
                type_name: std::any::type_name::<U>().to_owned(),
            }
        })
    }

    async fn make_request<T, U, V, F, E>(
        &self,
        path: &str,
        method: &str,
        request: T,
        map: F,
    ) -> Result<V, FFIBridgeError>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
        F: Fn(U) -> Result<V, E>,
        E: Into<FFIBridgeError>,
    {
        // JSON serialize request into body bytes
        let body = to_vec(&request).unwrap();

        // Append relative path to base url
        let url = format!("localhost:3000/v1/{}", path.to_owned());

        // Create Network request object, which will be translated by
        // Swift side into a `[Swift]URLRequest`
        let request = FFINetworkingRequest {
            url,
            body,
            method: method.to_owned(),
            headers: HashMap::<String, String>::from_iter([(
                "Content-Type".to_owned(),
                "application/json".to_owned(),
            )]),
        };

        // Let Swift side make network request and await response
        let response = self.networking_dispatcher.dispatch(request).await?;

        // Read out HTTP body from response and JSON parse it into U
        let model = self
            .model_from_response(response)
            .map_err(|error| FFIBridgeError::FromRust { error })?;
        // return model
        // Map U -> V
        map(model).map_err(|e| e.into())
    }

    pub(crate) async fn post<T, U, V, F, E>(
        &self,
        path: &str,
        request: T,
        map: F,
    ) -> Result<V, FFIBridgeError>
    where
        T: Serialize,
        U: for<'a> Deserialize<'a>,
        F: Fn(U) -> Result<V, E>,
        E: Into<FFIBridgeError>,
    {
        self.make_request(path, "POST", request, map).await
    }
}

pub trait IsOutcomeListener: From<FFIOperationOutcomeListener<Self::Outcome>> {
    type Request;
    type Response;
    type Failure: Into<FFISideError>;
    type Outcome: Into<Result<Self::Response, Self::Failure>>;
}


//     #[export]
// pub async fn get_aggregated_list(&self) -> Result<Vec<AggregatedCoinInformation>, FFIBridgeError> {
//     self.make_request(
//         ListOfCoinsRequest::new(10),
//         "localhost:3000/v1/coins/list/aggregated",
//         "POST"
//     )
//     .await
// }
// }

pub struct FFIOperationDispatcher<L: IsOutcomeListener> {
    pub executor: Arc<dyn FFIOperationExecutor<L>>,
}

/// An object representing that Rust is listening on the result of an operation
/// carried out by FFI (Swift-side). When FFI side has finished the operation,
/// either successfully or with failure, it passes back this result to Rust
/// side by calling `notify_outcome`. This is effectively a callback pattern.
pub struct FFIOperationOutcomeListener<R> {
    sender: Mutex<Option<Sender<R>>>,
}

impl<R> FFIOperationOutcomeListener<R> {
    pub(crate) fn new(sender: Sender<R>) -> Self {
        Self {
            sender: Mutex::new(Some(sender)),
        }
    }

    /// This is called from FFI Side (Swift side), inside the implementation of
    /// an `execute_request:operation:listener_rust_side` method on a [`FFIOperationExecutor`],
    /// when the operation has finished, with the `result` of type Self::R
    pub(crate) fn notify_outcome(&self, result: R) {
        self.sender
            .lock()
            .expect("Should only have access sender Mutex once.")
            .take()
            .expect("You MUST NOT call `notifyOutcome` twice in Swift.")
            .send(result)
            .map_err(|_| RustSideError::FailedToPropagateResultFromFFIOperationBackToDispatcher)
            .expect("Must never fail, since some context's in FFI side cannot be throwing.")
    }
}

#[uniffi::export(with_foreign)]
pub trait FFINetworkingExecutor: FFIOperationExecutor<FFINetworkingOutcomeListener> {
    fn execute_networking_request(
        &self,
        request: FFINetworkingRequest,
        listener_rust_side: Arc<FFINetworkingOutcomeListener>,
    ) -> Result<(), FFINetworkingError>;
}

impl<U: FFINetworkingExecutor> FFIOperationExecutor<FFINetworkingOutcomeListener> for U {
    fn execute_request(
        &self,
        request: <FFINetworkingOutcomeListener as IsOutcomeListener>::Request,
        listener_rust_side: FFINetworkingOutcomeListener,
    ) -> Result<(), FFINetworkingError> {
        self.execute_networking_request(request, listener_rust_side.into())
    }
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

// #[derive(Debug, uniffi::Error, Clone)]
// pub enum FFINetworkingError {
//     FailedToNotifyOutcome { string: String },
//     FailedToExecuteRequest { string: String }
// }

// impl std::fmt::Display for FFINetworkingError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             FFINetworkingError::FailedToExecuteRequest {string} => write!(f, "{}", string),
//             FFINetworkingError::FailedToNotifyOutcome {string} => write!(f, "{}", string),
//         }
//     }
// }

// impl std::error::Error for FFINetworkingError {
//     fn description(&self) -> &str {
//         match self {
//             FFINetworkingError::FailedToExecuteRequest { string } => string,
//             FFINetworkingError::FailedToNotifyOutcome{ string } => string,
//         }
//     }
// }

pub trait FFIOperationExecutor<L: IsOutcomeListener>: Send + Sync {
    fn execute_request(
        &self,
        request: L::Request,
        listener_rust_side: L,
    ) -> Result<(), FFINetworkingError>;
}

#[derive(Object)]
pub struct FFINetworkingOutcomeListener {
    result_listener: FFIOperationOutcomeListener<FFINetworkingOutcome>,
}
impl IsOutcomeListener for FFINetworkingOutcomeListener {
    type Request = FFINetworkingRequest;
    type Response = FFINetworkingResponse;
    type Failure = FFINetworkingError;
    type Outcome = FFINetworkingOutcome;
}

impl From<FFIOperationOutcomeListener<FFINetworkingOutcome>> for FFINetworkingOutcomeListener {
    fn from(value: FFIOperationOutcomeListener<FFINetworkingOutcome>) -> Self {
        Self::with_result_listener(value)
    }
}

#[export]
impl FFINetworkingOutcomeListener {
    fn notify_outcome(&self, result: FFINetworkingOutcome) {
        self.result_listener.notify_outcome(result.into())
    }
}

#[derive(Enum, Clone, Debug)]
pub enum FFINetworkingOutcome {
    Success { value: FFINetworkingResponse },
    Failure { error: FFINetworkingError },
}

impl Into<Result<FFINetworkingResponse, FFINetworkingError>> for FFINetworkingOutcome {
    fn into(self) -> Result<FFINetworkingResponse, FFINetworkingError> {
        match self {
            Self::Success { value } => Ok(value),
            Self::Failure { error } => Err(error),
        }
    }
}

impl FFINetworkingOutcomeListener {
    pub fn with_result_listener(
        result_listener: FFIOperationOutcomeListener<FFINetworkingOutcome>,
    ) -> Self {
        Self { result_listener }
    }
}

impl<L: IsOutcomeListener> FFIOperationDispatcher<L> {
    pub fn new(handler: Arc<dyn FFIOperationExecutor<L>>) -> Self {
        Self { executor: handler }
    }

    pub(crate) async fn dispatch(
        &self,
        operation: L::Request,
    ) -> Result<L::Response, FFIBridgeError> {
        // Underlying tokio channel used to get result from Swift back to Rust.
        let (sender, receiver) = channel::<L::Outcome>();

        // Our callback we pass to Swift
        let outcome_listener = FFIOperationOutcomeListener::new(sender);

        // Make request
        self.executor
            .execute_request(
                // Pass operation to Swift to make
                operation,
                // Pass callback, Swift will call `result_listener.notify_outcome`
                outcome_listener.into(),
            )
            .map_err(|e| FFIBridgeError::FromFFI { error: e.into() })?;

        // Await response from Swift
        let response = receiver.await.map_err(|_| FFIBridgeError::FromRust {
            error: RustSideError::FailedToReceiveResponseFromSwift,
        })?;

        response.into().map_err(|e| e.into().into())
    }
}
