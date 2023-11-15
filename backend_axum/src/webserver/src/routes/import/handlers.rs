use axum::extract::Path;
use service::import::import_service;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::Json;

pub async fn import(   
    Path(input): Path<(String, String)>
) -> Result<Json<String>, ResponseError> {
    prepare_response(import_service::import_file(input.0, input.1).await)
}

