use std::collections::HashMap;

use super::api_client::{ApiClient, Client};
use axum::http::StatusCode;
use serde::{de::DeserializeOwned, Serialize};

impl<T: Client> ApiClient<T> {
    pub async fn get<Request, Response>(
        client_source: T,
        path: &str,
        payload: Request,
    ) -> Result<(StatusCode, axum::Json<Response>), (StatusCode, axum::Json<String>)>
    where
        <Request as QueryItems>::Query: Serialize,
        Request: QueryItems + std::fmt::Debug,
        Response: DeserializeOwned,
    {
        let mut url = client_source.get_base_url();
        url.push_str(path);

        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .headers(client_source.get_headers())
            .query(&payload.get_all_queries())
            .send()
            .await
            .unwrap()
            .json::<Response>()
            .await;

        // match response. .status() {
        //     reqwest::StatusCode::OK => Ok((StatusCode::OK, axum::Json::<Response>(response) )),
        //     _other => Err(StatusCode::INTERNAL_SERVER_ERROR),
        // }

        match response {
            Ok(json) => Ok((StatusCode::OK, axum::Json::<Response>(json))),
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(err.to_string()),
            )),
        }
    }
}

pub trait QueryItems {
    type Query;
    fn get_all_queries(&self) -> HashMap<&str, Self::Query>;
}
