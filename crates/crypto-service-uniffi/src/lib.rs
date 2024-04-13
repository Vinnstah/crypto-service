#![feature(trait_upcasting)]

pub mod alphavantage_service;
pub mod client_trait;
pub mod api_client;
pub mod coin_watch_service;
pub mod network_antenna;

uniffi::include_scaffolding!("crypto_service");
