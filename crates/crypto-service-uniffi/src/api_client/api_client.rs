use std::{collections::HashMap, sync::Arc};

use anyhow::Error;
use reqwest::header::HeaderMap;
use uniffi::{check_remaining, metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer};

use crate::client_trait::QueryItems;


pub struct ApiClient {
    client: Arc<reqwest::Client>,
}

impl ApiClient {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new().into(),
        }
    }
}

// #[uniffi::export]
// impl ApiClient {
//     pub fn counstruct_request<T, C: DefineClient>(
//         client_source: C,
//         path: Path,
//         query: Box<dyn QueryItems<Query = T>>
//     ) -> Result<Request, (StatusCode, axum::Json<String>)>
//     {
//         let mut url = client_source.get_base_url();
//         url.push_str(path);

//         C::set_client()
//         .get(url)
//         .headers(client_source.get_headers())
//         .query(&query)
//         .build()
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(e.to_string())))
//     }
// }

// #[derive(uniffi::Enum)]
// pub enum Path {
//     Orderbook
// }

// #[uniffi::export]
// pub trait DefineClient: Send + Sync {
//     fn set_client(&self) -> Arc<Box<reqwest::Client>>;
// }
