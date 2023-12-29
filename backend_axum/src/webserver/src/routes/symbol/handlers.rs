use crate::utils::app_state::AppState;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::extract::State;
use axum::Json;
use service::symbol::symbol_dto::SymbolDto;
use service::symbol::symbol_service;
use tracing::info;

pub async fn get_symbols(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<SymbolDto>>, ResponseError> {
    info!("Incoming Request: get_symbols");
    let result = symbol_service::get_symbols(&app_state.pool, app_state.redis).await;
    prepare_response(result)
}
