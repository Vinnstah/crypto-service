pub mod api_client;
pub mod binance_service;
pub mod coinapi_service;
pub mod state;

uniffi::include_scaffolding!("crypto_service");
