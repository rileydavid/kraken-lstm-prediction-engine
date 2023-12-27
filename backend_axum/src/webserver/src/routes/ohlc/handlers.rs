use std::sync::Arc;

use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use crate::webserver::Testing;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use axum::http::StatusCode;
use rustis::client::Client;
use service::ohlc::ohlc_dto::OhlcDto;
use service::ohlc::ohlc_service;
use tracing::info;

use crate::routes::ohlc::dto::OhlcRequestDto;

pub async fn get_ohlc_hour_symbol_interval(
    Path(input): Path<(i32, i32)>,
    State(redis): State<Arc<Client>>,
) -> Result<Json<Vec<OhlcDto>>, ResponseError> {
    let result = ohlc_service::get_ohlc_hour_symbol_interval(input.0, input.1, redis).await;
    prepare_response(result)
}

pub async fn get_ohlc_day_symbol_interval(
    Path(input): Path<(i32, i32)>,
) -> Result<Json<Vec<OhlcDto>>, ResponseError> {
    let result = ohlc_service::get_ohlc_day_symbol_interval(input.0, input.1).await;
    prepare_response(result)
}

pub async fn post_ohlc_hour_start_date(
    State(redis): State<Arc<Client>>,
    Json(payload): Json<OhlcRequestDto>
) -> Result<Json<Vec<OhlcDto>>, ResponseError> {

    if payload.interval.is_none() {
        return Err(ResponseError::new(StatusCode::BAD_REQUEST, "400".into(), "to_date is required".to_string()));
    }

    let result = ohlc_service::get_ohlc_hour_start_date(
        payload.symbol_id,
        payload.interval.unwrap(),
        payload.from_date,
        redis,
    )
    .await;
    prepare_response(result)
}

pub async fn post_ohlc_day_start_date(
    Json(payload): Json<OhlcRequestDto>,
) -> Result<Json<Vec<OhlcDto>>, ResponseError> {

    if payload.interval.is_none() {
        return Err(ResponseError::new(StatusCode::BAD_REQUEST, "400".into(), "to_date is required".to_string()));
    }

    let result = ohlc_service::get_ohlc_day_start_date(
        payload.symbol_id,
        payload.interval.unwrap(),
        payload.from_date,
    )
    .await;
    prepare_response(result)
}

pub async fn post_ohlc_hour_range(
    Json(payload): Json<OhlcRequestDto>,
) -> Result<Json<Vec<OhlcDto>>, ResponseError> {
    info!("Incoming Request: post_ohlc_hour_range - to {:?}, from {:?}", payload.to_date, payload.from_date);

    if payload.to_date.is_none() {
        return Err(ResponseError::new(StatusCode::BAD_REQUEST, "400".into(), "to_date is required".to_string()));
    }

    let result = ohlc_service::get_ohlc_hour_range(
        payload.symbol_id,
        payload.from_date,
        payload.to_date.unwrap(),
    )
    .await;
    prepare_response(result)
}
