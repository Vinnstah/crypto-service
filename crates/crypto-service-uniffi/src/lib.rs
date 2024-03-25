use std::sync::Arc;
use crate::client_trait::Client;
pub mod binance_service;
pub mod alphavantage_service;
pub mod client_trait;
pub mod api_client;
pub mod coin_watch_service;

uniffi::include_scaffolding!("crypto_service");
