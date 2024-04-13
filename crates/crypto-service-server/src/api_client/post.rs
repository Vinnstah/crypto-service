use std::{collections::HashMap, str::FromStr};

use super::api_client::{ApiClient};
use axum::http::StatusCode;
use crypto_service::client_trait::Client;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Request,
};
use serde::{de::DeserializeOwned, Serialize};

impl ApiClient {
    pub async fn post<U, C: Client, R: Serialize>(
        &self,
        client_source: C,
        path: &str,
        body: R,
    ) -> Result<
        (StatusCode, axum::Json<U>),
        (StatusCode, axum::Json<String>),
    >
    where
        U: DeserializeOwned,
    {
        let request = self.counstruct_post_request(
            client_source,
            path,
            body,
        )?;
        let response_bytes =
            self.execute_request(request).await?;
        println!("{:#?}", response_bytes);
        self.deserialize_response::<U>(response_bytes).await
    }

    fn counstruct_post_request<C: Client, R: Serialize>(
        &self,
        client_source: C,
        path: &str,
        body: R,
    ) -> Result<Request, (StatusCode, axum::Json<String>)>
    {
        let mut url = client_source.get_base_url();
        url.push_str(path);

        self.http_client
            .post(url)
            .json(&body)
            .headers(Headers::from(client_source.get_headers()).0)
            .build()
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(e.to_string()),
                )
            })
    }
}

pub struct Headers(pub HeaderMap);

impl From<HashMap<String, String>> for Headers {
    fn from(value: HashMap<String, String>) -> Self {
        let mut map: HeaderMap<HeaderValue> =
            HeaderMap::new();
        value.iter().map(|(k, v)| {
            map.append(
                HeaderName::from_str(k.as_str())
                    .expect("msg"),
                v.parse().expect("Unwrapping value failed"),
            )
        });
        Headers(map)
    }
}
