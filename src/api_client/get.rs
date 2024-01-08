use super::{api_client::ApiClient, client_trait::Client};
use axum::http::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

impl ApiClient {
    pub async fn get<Request, Response, T: Client>(
        &self,
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

        let response = self
            .http_client
            .get(url)
            .headers(client_source.get_headers())
            .query(&payload.get_all_queries())
            .send()
            .await
            .unwrap()
            .json::<Response>()
            .await;

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
