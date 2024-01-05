use std::collections::HashSet;

use crate::utils::app_state::AppState;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use axum::http::StatusCode;
use service::symbol::symbol_dto::SymbolDto;
use service::symbol::symbol_service;
use tracing::info;
use cache::repository::symbol_cache;

pub async fn get_symbols(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<SymbolDto>>, ResponseError> {
    info!("Incoming Request: get_symbols");
    let result = symbol_service::get_symbols(&app_state.pool, app_state.redis).await;
    prepare_response(result)
}

pub async fn get_available_symbols() -> Result<Json<Vec<SymbolDto>>, ResponseError> {
    let result = symbol_service::get_available_symbols().await;
    prepare_response(result)
}


// TODO: move this to the service!
// always adds symbol/USD
pub async fn get_add_symbol(
    State(app_state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<String>, ResponseError> {
    info!("Incoming Request: get_add_symbol {}", symbol);
    let result = symbol_service::add_subscription_symbol(&app_state.sender, symbol).await;
   prepare_response(result)
}
