use axum::extract::Path;
use service::export::export_service;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::Json;

pub async fn export(   
    Path(symbol): Path<String>
) -> Result<Json<String>, ResponseError> {
    prepare_response(export_service::export_file(symbol.to_string()).await)
}

