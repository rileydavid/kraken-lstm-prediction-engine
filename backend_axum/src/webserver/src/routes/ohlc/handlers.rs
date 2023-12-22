use std::sync::Arc;

use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use crate::webserver::Testing;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use rustis::client::Client;
use service::ohlc::ohlc_dto::OhlcDto;
use service::ohlc::ohlc_service;

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
    let result = ohlc_service::get_ohlc_hour_start_date(
        payload.symbol_id,
        payload.interval,
        payload.start_date,
        redis,
    )
    .await;
    prepare_response(result)
}

pub async fn post_ohlc_day_start_date(
    Json(payload): Json<OhlcRequestDto>,
) -> Result<Json<Vec<OhlcDto>>, ResponseError> {
    let result = ohlc_service::get_ohlc_day_start_date(
        payload.symbol_id,
        payload.interval,
        payload.start_date,
    )
    .await;
    prepare_response(result)
}
