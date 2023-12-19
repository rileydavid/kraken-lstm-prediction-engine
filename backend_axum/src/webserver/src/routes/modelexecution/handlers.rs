use service::modelexecution::modelexecution_service;
use service::modelexecution::prediction_dto::PredictionResponseDto;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::Json;
use super::dto::ModelExecutionRequestDto;

pub async fn post_execute_model(Json(payload): Json<ModelExecutionRequestDto>) -> Result<Json<PredictionResponseDto>, ResponseError> {
    let result = modelexecution_service::execute_model( payload.start_date, payload.symbol_id).await;
    prepare_response(result)
}

pub async fn get_test() -> Result<Json<String>, ResponseError> {
    let result = Ok(String::from("ok")); 
    prepare_response(result)    
}

