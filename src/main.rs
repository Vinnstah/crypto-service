use anyhow::Result;
use axum::{routing::get, Router};
use crypto_service::{
    api_client::api_client::ApiClient,
    binance_service::{
        binance_client::BinanceClient,
        orderbook_handler::{self},
    },
    coinapi_service::{self, assets_handler, coinapi_client::CoinApiClient},
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

    Ok(())
}
