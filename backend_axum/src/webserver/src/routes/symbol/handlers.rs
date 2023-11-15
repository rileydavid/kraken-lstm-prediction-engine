use service::symbol::symbol_service;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::Json;
use service::symbol::symbol_dto::SymbolDto;

pub async fn get_symbols() -> Result<Json<Vec<SymbolDto>>, ResponseError> {
    let result = symbol_service::get_symbols().await;
    prepare_response(result)
}
