
use core::fmt::Debug;
use reqwest::header::{HeaderMap, ACCEPT};

use std::{env};



#[derive(PartialEq, Debug, Clone)]
pub struct CoinApiClient {
    pub headers: HeaderMap,
    pub base_url: String,
}

impl Default for CoinApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl CoinApiClient {
    pub fn new() -> Self {
        Self {
            headers: {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "X-CoinAPI-Key",
                    env::var("COINAPI_API_KEY")
                        .expect("No API-key found")
                        .parse()
                        .expect("Failed to parse header"),
                );
                headers.insert(
                    ACCEPT,
                    "application/json".parse().expect("Failed to parse header"),
                );
                headers
            },
            base_url: "https://rest.coinapi.io/v1/".to_string(),
        }
    }
}
//     async fn get<Request, Response>(
//         &self,
//         path: &str,
//         payload: Request,
//     ) -> Result<(StatusCode, reqwest::Response), StatusCode>
//     where
//         <Request as CoinAPIQueryItems>::Query: Serialize,
//         Request: CoinAPIQueryItems + Debug,
//     {
//         let mut url = self.base_url.clone();
//         url.push_str(path);

//         let client = reqwest::Client::new();

//         let response = client
//             .get(url)
//             .headers(self.headers.clone())
//             .query(&payload.get_all_parameters())
//             .send()
//             .await
//             .unwrap();

//         match response.status() {
//             reqwest::StatusCode::OK => Ok((StatusCode::OK, response)),
//             _other => Err(StatusCode::INTERNAL_SERVER_ERROR),
//         }
//     }
// }

// #[axum::debug_handler]
// pub async fn get_asset_icons(
//     extract::State(state): extract::State<AppState>,
//     axum::Json(payload): axum::Json<AssetIconsRequest>,
// ) -> Result<
//     (axum::http::StatusCode, axum::Json<Vec<AssetIcons>>),
//     (axum::http::StatusCode, axum::Json<String>),
// > {
//     let path = String::from_str("assets/icons/").unwrap() + &payload.size.to_string();
//     let response: Result<(StatusCode, reqwest::Response), StatusCode> = state
//         .coinapi_client
//         .get::<AssetIconsRequest, reqwest::Response>(&path, payload)
//         .await;

//     match response {
//         Ok(body) => Ok((
//             body.0,
//             axum::Json(body.1.json::<Vec<AssetIcons>>().await.unwrap()),
//         )),
//         Err(err) => Err((err, axum::Json(err.to_string()))),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_coinapi_client() {
        std::env::set_var("COINAPI_API_KEY", "Test Key");
        let coinapi_client = CoinApiClient::new();

        assert_eq!(
            coinapi_client.headers.get("X-CoinAPI-Key").unwrap(),
            "Test Key"
        );
        assert_eq!(
            coinapi_client.headers.get("ACCEPT").unwrap(),
            "application/json"
        );
        assert_eq!(coinapi_client.base_url, "https://rest.coinapi.io/v1/")
    }
}
