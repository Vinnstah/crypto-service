use anyhow::Result;
use axum::{extract::Query, routing::get, Router};
use crypto_service::binance_service::helpers::OrderBookResponse;
use crypto_service::binance_service::orderbook_handler::{get_order_book, Params};
use crypto_service::UniFfiTag;
use crypto_service::{
    api_client::api_client::ApiClient,
    binance_service::{binance_client::BinanceClient, orderbook_handler},
    coinapi_service::{
        self,
        assets_handler::{self, get_symbols},
        coinapi_client::CoinApiClient,
        helpers::{SymbolsParams, SymbolsResponse},
    },
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let binance_client: BinanceClient = BinanceClient::new();
    let coinapi_client: CoinApiClient = CoinApiClient::new();
    let api_client = ApiClient::new();

    let state = AppState::new(binance_client, coinapi_client, api_client);

    let app = Router::new()
        .route("/v1/orderbooks", get(orderbook_handler::get_order_book))
        .route("/v1/trades", get(orderbook_handler::get_recent_trades))
        .route("/v1/symbols/icons", get(assets_handler::get_asset_icons))
        .route("/v1/symbols", get(assets_handler::get_symbols))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    // todo!("Split into 2 crates under a workspace like: https://github.com/radixdlt/radix-engine-toolkit/tree/main");
    Ok(())
}

/// APA BANAN
#[uniffi::export]
pub async fn get_symbols_binding(params: SymbolsParams) -> Vec<SymbolsResponse> {
    let binance_client: BinanceClient = BinanceClient::new();
    let coinapi_client: CoinApiClient = CoinApiClient::new();
    let api_client = ApiClient::new();

    let state = AppState::new(binance_client, coinapi_client, api_client);

    get_symbols(
        axum::extract::State(state),
        Query::from(axum::extract::Query(params)),
    )
    .await
    .map(|x| x.1 .0)
    .unwrap()
}


#[uniffi::export]
pub async fn get_orderbook_binding(params: Params) -> OrderBookResponse {
    let binance_client: BinanceClient = BinanceClient::new();
    let coinapi_client: CoinApiClient = CoinApiClient::new();
    let api_client = ApiClient::new();
    let state = AppState::new(binance_client, coinapi_client, api_client);

    get_order_book(
        axum::extract::State(state),
        Query::from(axum::extract::Query(params)),
    )
    .await
    .map(|r| r.1 .0)
    .expect("Failed to get Orderbook")
}
