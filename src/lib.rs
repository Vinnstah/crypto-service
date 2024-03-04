pub mod api_client;
pub mod binance_service;
pub mod coinapi_service;
pub mod state;
pub mod coin_watch_service;

uniffi::include_scaffolding!("crypto_service");
