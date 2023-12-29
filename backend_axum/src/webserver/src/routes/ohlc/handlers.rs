use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::Json;
use axum::extract::State;
use service::ohlc::ohlc_dto::OhlcDto;
use service::ohlc::ohlc_service;
use sqlx::PgPool;
use tracing::info;

use crate::routes::ohlc::dto::OhlcRequestDto;

pub async fn post_ohlc_hour_range(
    State(pool): State<PgPool>,
    Json(payload): Json<OhlcRequestDto>,
) -> Result<Json<Vec<OhlcDto>>, ResponseError> {
    info!("Incoming Request: post_ohlc_hour_range - to {:?}, from {:?}", payload.to_date, payload.from_date);

    let result = ohlc_service::get_ohlc_hour_range(
        &pool,
        payload.symbol_id,
        payload.from_date,
        payload.to_date
    )
    .await;
    prepare_response(result)
}

pub async fn post_ohlc_day_range(
    State(pool): State<PgPool>,
    Json(payload): Json<OhlcRequestDto>,
) -> Result<Json<Vec<OhlcDto>>, ResponseError> {
    info!("Incoming Request: post_ohlc_day_range - to {:?}, from {:?}", payload.to_date, payload.from_date);

    let result = ohlc_service::get_ohlc_day_range(
        &pool,
        payload.symbol_id,
        payload.from_date,
        payload.to_date,
    )
    .await;
    prepare_response(result)
}



