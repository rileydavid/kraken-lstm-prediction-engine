use axum::extract::Path;
use axum::extract::State;
use service::import::import_service;
use sqlx::PgPool;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::Json;

pub async fn import(   
    State(pool): State<PgPool>,
    Path(input): Path<(String, String)>
) -> Result<Json<String>, ResponseError> {
    prepare_response(import_service::import_file(&pool,input.0, input.1).await)
}

