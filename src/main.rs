use anyhow::Result;
use axum::{routing::get, Router};
use crypto_service::{
    binance_service::{
        binance_client::BinanceClient,
        orderbook_handler::{self},
    },
    coinapi_service::{self, coinapi_client::CoinApiClient},
    state::AppState, api_client::api_client::ApiClient,
};

// pub struct AppState {
//     binance_client: BinanceClient,
//     coinapi_client: CoinApiClient
// }

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // let binance_client = ApiClient::new(BinanceClient::new());
    // let coinapi_client = ApiClient::new(CoinApiClient::new());
    let binance_client: BinanceClient = BinanceClient::new();
    let coinapi_client: CoinApiClient = CoinApiClient::new();
    let state = AppState {
        binance_client,
        coinapi_client,
    };

    let app = Router::new()
        // .route("/v1/orderbook", get(orderbook_handler::get_order_book))
        .route("/v1/trades", get(orderbook_handler::get_recent_trades))
        .route(
            "/v1/assets",
            get(coinapi_service::coinapi_client::get_asset_icons),
        )
        .with_state(state);
    // .with_state(coinapi_client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
