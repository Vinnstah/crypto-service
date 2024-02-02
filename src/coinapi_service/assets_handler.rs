// use axum::extract;

// use crate::{
//     coinapi_service::helpers::{AssetIcons, AssetIconsRequest, SymbolsRequest, SymbolsResponse},
//     state::AppState,
// };

// #[axum::debug_handler]
// pub async fn get_asset_icons(
//     extract::State(state): extract::State<AppState>,
//     axum::Json(payload): axum::Json<AssetIconsRequest>,
// ) -> Result<
//     (axum::http::StatusCode, axum::Json<Vec<AssetIcons>>),
//     (axum::http::StatusCode, axum::Json<String>),
// > {
//     state
//         .api_client
//         .get(state.coinapi_client, "assets/icons/", Some(payload))
//         .await
// }

// #[axum::debug_handler]
// pub async fn get_symbols(
//     extract::State(state): extract::State<AppState>,
//     axum::Json(payload): axum::Json<SymbolsRequest>,
// ) -> Result<
//     (axum::http::StatusCode, axum::Json<Vec<SymbolsResponse>>),
//     (axum::http::StatusCode, axum::Json<String>),
// > {
//     state
//         .api_client
//         .get(state.coinapi_client, "symbols", Some(payload))
//         .await
// }
