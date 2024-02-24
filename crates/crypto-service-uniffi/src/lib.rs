use std::sync::Arc;
use crate::client_trait::Client;
pub mod binance_service;
pub mod coinapi_service;
pub mod client_trait;
pub mod api_client;

uniffi::include_scaffolding!("crypto_service");
