use super::api_client::ApiClient;
use axum::http::StatusCode;
use crypto_service::client_trait::{Client, QueryItems};
use reqwest::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

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
            .headers(client_source.get_headers())
            .query(&query)
            .build()
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())))
    }

    // fn placeholder_binance_client_request() -> Request {
    //     env::set_var("BINANCE_API_KEY", "Bearer Key");
    //     let api_client = ApiClient::new();
    //     let binance_client = BinanceClient::new();
    //     let raw_payload = r#"{
    //         "symbol": "ETHBTC", "limit": 10
    //     }"#;
    //     let payload: OrderBookRequest = serde_json::from_str(raw_payload).unwrap();
    //     ApiClient::counstruct_request(&api_client, binance_client, "path", Some(payload))
    //         .expect("Failed")
    // }
}

#[cfg(test)]
mod tests {
    use std::{any::Any, str::FromStr};

    use crypto_service::binance_service::models::OrderBook;
    use reqwest::Method;
    use serde::de::IntoDeserializer;
    use serde_json::Number;

    use super::*;

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

    #[test]
    fn deserialize_response_json() {
        let response = r#"
        {
            "asks": [
              [
                "0.05916000",
                "8.77050000"
              ],
              [
                "0.05917000",
                "23.15060000"
              ],
              [
                "0.05918000",
                "30.24440000"
              ],
              [
                "0.05919000",
                "20.60540000"
              ],
              [
                "0.05920000",
                "41.42670000"
              ],
              [
                "0.05921000",
                "37.61000000"
              ],
              [
                "0.05922000",
                "48.83790000"
              ],
              [
                "0.05923000",
                "28.23870000"
              ],
              [
                "0.05924000",
                "1.41050000"
              ],
              [
                "0.05925000",
                "36.20100000"
              ]
            ],
            "bids": [
              [
                "0.05915000",
                "19.49480000"
              ],
              [
                "0.05914000",
                "24.62770000"
              ],
              [
                "0.05913000",
                "20.42650000"
              ],
              [
                "0.05912000",
                "32.78410000"
              ],
              [
                "0.05911000",
                "22.14670000"
              ],
              [
                "0.05910000",
                "36.17620000"
              ],
              [
                "0.05909000",
                "16.09100000"
              ],
              [
                "0.05908000",
                "5.97210000"
              ],
              [
                "0.05907000",
                "26.93540000"
              ],
              [
                "0.05906000",
                "14.21390000"
              ]
            ],
            "lastUpdateId": 7038480085
          }"#;
        let deserialized_response: OrderBook = serde_json::from_str(response).unwrap();
        assert_eq!(deserialized_response.last_update_id, 7038480085);
        assert_eq!(deserialized_response.asks.len(), 10);
        assert_eq!(deserialized_response.bids.len(), 10);
        // assert_eq!(
        //     deserialized_response.asks,
        //     [
        //         ["0.05916000", "8.77050000"],
        //         ["0.05917000", "23.15060000"],
        //         ["0.05918000", "30.24440000"],
        //         ["0.05919000", "20.60540000"],
        //         ["0.05920000", "41.42670000"],
        //         ["0.05921000", "37.61000000"],
        //         ["0.05922000", "48.83790000"],
        //         ["0.05923000", "28.23870000"],
        //         ["0.05924000", "1.41050000"],
        //         ["0.05925000", "36.20100000"]
        //     ]
        // );
        // assert_eq!(
        //     deserialized_response.bids,
        //     [
        //         ["0.05915000", "19.49480000"],
        //         ["0.05914000", "24.62770000"],
        //         ["0.05913000", "20.42650000"],
        //         ["0.05912000", "32.78410000"],
        //         ["0.05911000", "22.14670000"],
        //         ["0.05910000", "36.17620000"],
        //         ["0.05909000", "16.09100000"],
        //         ["0.05908000", "5.97210000"],
        //         ["0.05907000", "26.93540000"],
        //         ["0.05906000", "14.21390000"]
        //     ]
        // );
    }
}
