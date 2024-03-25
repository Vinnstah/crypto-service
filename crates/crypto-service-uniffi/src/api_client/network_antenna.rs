
use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::oneshot::Sender;
use uniffi::{export, Enum, Object, Record};

pub trait NetworkAntenna {}


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
            .map_err(|_| UniffiError::FailedToNotifyOutcome {string: "string".into() })
            .expect("Must never fail, since some context's in FFI side cannot be throwing.")
    }
}

#[uniffi::export(with_foreign)]
pub trait FFINetworkingExecutor: FFIOperationExecutor<FFINetworkingOutcomeListener> {
    fn execute_networking_request(
        &self,
        request: FFINetworkRequest,
        listener_rust_side: Arc<FFINetworkingOutcomeListener>,
    ) -> Result<(), UniffiError>;
}

impl<U: FFINetworkingExecutor> FFIOperationExecutor<FFINetworkingOutcomeListener> for U {
    fn execute_request(
        &self,
        request: <FFINetworkingOutcomeListener as IsOutcomeListener>::Request,
        listener_rust_side: FFINetworkingOutcomeListener,
    ) -> Result<(), UniffiError> {
        self.execute_networking_request(request, listener_rust_side.into())
    }
}

#[derive(uniffi::Record)]
pub struct FFINetworkRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

#[derive(Debug, uniffi::Error, Clone)]
pub enum UniffiError {
    FailedToNotifyOutcome { string: String },
    FailedToExecuteRequest { string: String }
}

impl std::fmt::Display for UniffiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UniffiError::FailedToExecuteRequest {string} => write!(f, "{}", string),
            UniffiError::FailedToNotifyOutcome {string} => write!(f, "{}", string),
        }
    }
}

impl std::error::Error for UniffiError {
    fn description(&self) -> &str {
        match self {
            UniffiError::FailedToExecuteRequest { string } => string,
            UniffiError::FailedToNotifyOutcome{ string } => string,
        }
    }
}

pub trait FFIOperationExecutor<L: IsOutcomeListener>: Send + Sync {
    fn execute_request(
        &self,
        request: L::Request,
        listener_rust_side: L,
    ) -> Result<(), UniffiError>;
}

pub trait IsOutcomeListener: From<FFIOperationOutcomeListener<Self::Outcome>> {
    type Request;
    type Response;
    type Failure: Into<UniffiError>;
    type Outcome: Into<Result<Self::Response, Self::Failure>>;
}
#[derive(Object)]
pub struct FFINetworkingOutcomeListener {
    result_listener: FFIOperationOutcomeListener<FFINetworkingOutcome>,
}
impl IsOutcomeListener for FFINetworkingOutcomeListener {
    type Request = FFINetworkRequest;
    type Response = FFINetworkingResponse;
    type Failure = UniffiError;
    type Outcome = FFINetworkingOutcome;
}

impl From<FFIOperationOutcomeListener<FFINetworkingOutcome>> for FFINetworkingOutcomeListener {
    fn from(value: FFIOperationOutcomeListener<FFINetworkingOutcome>) -> Self {
        Self::with_result_listener(value)
    }
}
impl FFINetworkingOutcomeListener {
    pub fn with_result_listener(
        result_listener: FFIOperationOutcomeListener<FFINetworkingOutcome>,
    ) -> Self {
        Self { result_listener }
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
    Failure { error: UniffiError },
}

impl Into<Result<FFINetworkingResponse, UniffiError>> for FFINetworkingOutcome {
    fn into(self) -> Result<FFINetworkingResponse, UniffiError> {
        match self {
            Self::Success { value } => Ok(value),
            Self::Failure { error } => Err(error),
        }
    }
}

#[derive(Record, Clone, Debug, PartialEq, Eq)]
pub struct FFINetworkingResponse {
    pub status_code: u16,

    /// Can be empty.
    pub body: Vec<u8>,
}