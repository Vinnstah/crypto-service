use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use crypto_service_server::{
    alphavantage_api::{
        alpha_client::AlphaAdvantageClient, alpha_handler,
    },
    api_client::api_client::ApiClient,
    binance::{
        binance_client::BinanceClient, orderbook_handler,
    },
    coin_watch::{
        coin_watch_client::CoinWatchClient,
        coin_watch_handlers,
    },
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let binance_client: BinanceClient =
        BinanceClient::new();
    let alpha_client: AlphaAdvantageClient =
        AlphaAdvantageClient::new();
    let coin_watch_client = CoinWatchClient::new();
    let api_client = ApiClient::new();

    let state = AppState::new(
        binance_client,
        alpha_client,
        coin_watch_client,
        api_client,
    );

    let app = Router::new()
        .route("/v1/orderbooks", get(orderbook_handler::get_order_book))
        .route("/v1/trades", get(orderbook_handler::get_recent_trades))
        .route("/v1/stocks", get(alpha_handler::get_top_gainers_and_losers))
        .route("/v1/coins/list", post(coin_watch_handlers::get_list_of_coins))
        .route("/v1/coins/single", post(coin_watch_handlers::get_coin_meta_info))
        .route("/v1/coins/single/history", post(coin_watch_handlers::get_coin_history_info))
        .route("/v1/coins/list/aggregated", post(coin_watch_handlers::get_aggregated_coin_list))
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
