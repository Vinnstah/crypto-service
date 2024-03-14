use std::{collections::HashMap, sync::Arc};

use anyhow::Error;
use axum::Json;
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde_json::Serializer;
use uniffi::{check_remaining, metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer};

use crate::{
    binance_service::models::OrderBook, client_trait::QueryItems,
    coinapi_service::models::SymbolsResponse,
};

/// protocol NetworkManager {
///     func install(self) -> some RequestHandler   
/// }
///
/// struct Coordinator: NetworkManager {
///     func install(self) -> some RequestHandler {
///         ApiClient(client: .....)
///

#[uniffi::export(callback_interface)]
pub trait NetworkAntenna: Send + Sync {
    fn send(&self, request: RequestCases) -> Result<Response, DataTaskFailure>;
}

#[derive(uniffi::Enum)]
pub enum RequestCases {
    Orderbook { orderbook: OrderBook },
    Symbols { symbolls: SymbolsResponse },
}

#[derive(uniffi::Enum)]
pub enum Response {
    Orderbook { orderbook: OrderBook },
    Symbols { symbolls: SymbolsResponse },
}


pub fn request(
    network_antenna: Box<dyn NetworkAntenna>,
    request: RequestCases,
) -> Result<Response, DataTaskFailure> {

    network_antenna.send(request)
}

// #[derive(uniffi::Record)]
// pub struct ApiClient {
//     client: Arc<dyn RequestHandler>,
// }

// impl ApiClient {
//     #[uniffi::constructor]
//     pub fn new(data_task: Arc<dyn RequestHandler>) -> Arc<Self> {
//         Arc::new(Self { client: data_task })
//     }
// }

#[derive(Debug, uniffi::Error)]
pub enum DataTaskFailure {
    Error
}

#[derive(uniffi::Record)]
pub struct Data {}

#[derive(uniffi::Record)]
pub struct URLRequest {}

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
