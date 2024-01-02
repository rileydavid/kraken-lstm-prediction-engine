use crate::utils::app_state::AppState;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use service::symbol::symbol_dto::SymbolDto;
use service::symbol::symbol_service;
use tracing::error;
use tracing::info;

pub async fn get_symbols(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<SymbolDto>>, ResponseError> {
    info!("Incoming Request: get_symbols");
    let result = symbol_service::get_symbols(&app_state.pool, app_state.redis).await;
    prepare_response(result)
}


pub async fn get_add_symbol(
    State(app_state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<String>, ResponseError> {
    info!("Incoming Request: get_add_symbol");

    let send = app_state.sender.send("XRP/USD".into()).await;

    if send.is_err() {
        error!("Sned Error");
    }

    return Ok(Json("Ok".to_string()));
}
