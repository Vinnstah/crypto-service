use super::{
    api_client::ApiClient,
    client_trait::{self, Client},
};
use axum::http::StatusCode;
use reqwest::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

impl ApiClient {
    pub async fn post<U, C: Client, R: Serialize>(
        &self,
        client_source: C,
        path: &str,
        body: R,
    ) -> Result<(StatusCode, axum::Json<U>), (StatusCode, axum::Json<String>)>
    where
        U: DeserializeOwned,
    {
        let request = self.counstruct_post_request(client_source, path, body)?;
        println!("{:#?}", request.try_clone());
        let response_bytes = self.execute_request(request).await?;
        println!("{:#?}", response_bytes);
        self.deserialize_response::<U>(response_bytes).await
    }

    fn counstruct_post_request<C: Client, R: Serialize>(
        &self,
        client_source: C,
        path: &str,
        body: R,
    ) -> Result<Request, (StatusCode, axum::Json<String>)> {
        let mut url = client_source.get_base_url();
        url.push_str(path);

        self.http_client
            .post(url)
            .json(&body)
            .headers(client_source.get_headers())
            .build()
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())))
    }
}
