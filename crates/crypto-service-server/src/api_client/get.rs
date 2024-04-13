
use super::api_client::ApiClient;
use axum::http::StatusCode;
use crypto_service::client_trait::{Client, QueryItems};
use reqwest::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};
use crate::api_client::post::Headers;

impl ApiClient {
    pub async fn get<T, U, C: Client>(
        &self,
        client_source: C,
        path: &str,
        query: T,
    ) -> Result<(StatusCode, axum::Json<U>), (StatusCode, axum::Json<String>)>
    where
        <T as QueryItems>::Query: Serialize,
        T: QueryItems + std::fmt::Debug + Serialize,
        U: DeserializeOwned,
    {
        let request = self.counstruct_request(client_source, path, query)?;

        let response_bytes = self.execute_request(request).await?;

        self.deserialize_response(response_bytes).await
    }

    pub async fn deserialize_response<U: DeserializeOwned>(
        &self,
        response_bytes: Response,
    ) -> Result<(StatusCode, axum::Json<U>), (StatusCode, axum::Json<String>)> {
        response_bytes
            .json::<U>()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())))
            .map(|r| (StatusCode::OK, axum::Json::<U>(r)))
    }

    pub async fn execute_request(
        &self,
        request: Request,
    ) -> Result<Response, (StatusCode, axum::Json<String>)> {
        println!("{:#?}", request);
        self.http_client
            .execute(request)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, axum::Json(e.to_string())))
    }

    fn counstruct_request<T, C: Client>(
        &self,
        client_source: C,
        path: &str,
        query: T,
    ) -> Result<Request, (StatusCode, axum::Json<String>)>
    where
        <T as QueryItems>::Query: Serialize,
        T: QueryItems + std::fmt::Debug + Serialize,
    {
        let mut url = client_source.get_base_url();
        url.push_str(path);

        self.http_client
            .get(url)
            .headers(Headers::from(client_source.get_headers()).0)
            .query(&query)
            .build()
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())))
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn construct_request() {
    //     let request = ApiClient::placeholder_binance_client_request();
    //     assert_eq!(request.method(), Method::GET);
    //     assert_eq!(request.headers().len(), 2);
    //     assert_eq!(request.headers().get("x-mbx-apikey").unwrap(), "Bearer Key");
    //     assert_eq!(
    //         request.headers().get("content-type").unwrap(),
    //         "application/x-www-form-urlencoded"
    //     );
    //     assert_eq!(request.url().path().to_string(), "/api/v3/path");
    //     assert_eq!(request.url().query().unwrap(), "symbol=ETHBTC&limit=10");
    // }

    // #[tokio::test]
    // #[cfg(not(tarpaulin))]
    // async fn execute_request() {
    //     let api_client = ApiClient::new();
    //     let request = ApiClient::placeholder_binance_client_request();
    //     let response = ApiClient::execute_request(&api_client, request)
    //         .await
    //         .expect("Failed 2");
    //     assert_eq!(response.status(), 404);
    // }

    // #[tokio::test]
    // async fn deserialize_response() {
    //     let api_client = ApiClient::new();
    //     let request = ApiClient::placeholder_binance_client_request();
    //     let response = ApiClient::execute_request(&api_client, request)
    //         .await
    //         .expect("Failed");
    //       let response = r#"
    //     {
    //         "asks": [
    //           [
    //             "0.05916000",
    //             "8.77050000"
    //           ]
    //         ],
    //         "bids": [
    //           [
    //             "0.05915000",
    //             "19.49480000"
    //           ]
    //         ],
    //         "lastUpdateId": 7038480085
    //       }"#;
    //     let deserialized_response = api_client.deserialize_response::<OrderBook>(response).await;
    //     assert_eq!(deserialized_response.is_ok(), true);
    // }

    // #[test]
    // fn get_orderbook() {
    //     std::env::set_var("BINANCE_API_KEY", "TEST");

    //     let api_client = ApiClient::new();
    //     let binance_client = BinanceClient::new();
    //     let orderbook_request = OrderBookRequest {
    //         symbol: serde_json::Value::String("ETHBTC".to_string()),
    //         limit: Some(serde_json::Value::Number(Number::from_str("10").unwrap())),
    //     };

    //     let response = api_client
    //         .get::<OrderBookRequest, OrderBook, binance_client::BinanceClient>(
    //             binance_client,
    //             "depth",
    //             orderbook_request,
    //         );
    // }
}
