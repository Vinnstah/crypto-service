use std::collections::HashMap;

use axum::http::StatusCode;
use serde::Serialize;
use super::api_client::{ApiClient, Client};

impl<T: Client> ApiClient<T> {
    pub async fn get<Request, Response>(
        client_source: T,
        path: &str,
        payload: Request,
    ) -> Result<(StatusCode, reqwest::Response), StatusCode>
    where
        <Request as QueryItems>::Query: Serialize,
        Request: QueryItems + std::fmt::Debug,
    {
        let mut url = client_source.get_base_url();
        url.push_str(path);

        let client = reqwest::Client::new();
        println!("{:#?}", payload);
        let response = client
            .get(url)
            .headers(client_source.get_headers())
            .query(&payload.get_all_queries())
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => Ok((StatusCode::OK, response)),
            _other => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub trait QueryItems {
    type Query;
    fn get_all_queries(&self) -> HashMap<&str, Self::Query>;
}

