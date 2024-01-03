use anyhow::Result;
use axum::{routing::get, Router};
use crypto_service::binance_service::orderbook_handler::{self, BinanceClient};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let client: BinanceClient = BinanceClient::new();

    let app = Router::new()
        .route("/v1/orderbook", get(orderbook_handler::get_order_book))
        .with_state(client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
