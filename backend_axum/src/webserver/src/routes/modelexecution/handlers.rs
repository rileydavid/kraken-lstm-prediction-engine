use super::dto::ModelExecutionRequestDto;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::extract::State;
use axum::Json;
use service::modelexecution::modelexecution_service;
use service::modelexecution::prediction_dto::PredictionResponseDto;
use sqlx::PgPool;
use tracing::info;

pub async fn post_execute_model(
    State(pool): State<PgPool>,
    Json(payload): Json<ModelExecutionRequestDto>,
) -> Result<Json<PredictionResponseDto>, ResponseError> {
    info!(
        "Incoming Request: post_execute_model - from {:?}, to {:?}",
        payload.from_date, payload.symbol_id
    );
    let result =
        modelexecution_service::execute_model(&pool, payload.from_date, payload.symbol_id).await;
    prepare_response(result)
}
