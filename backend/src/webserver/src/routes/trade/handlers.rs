use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::Json;
use axum::extract::State;
use service::trade::trade_dto::TradeDto;
use service::trade::trade_service;
use sqlx::PgPool;
use tracing::info;

use super::dto::TradeRequestDto;

pub async fn get_trades_range(
    State(pool): State<PgPool>,
    Json(payload): Json<TradeRequestDto>,
) -> Result<Json<Vec<TradeDto>>, ResponseError> {
    info!(
        "Incoming Request: get_trades_range - from {:?}, to {:?}",
        payload.from_date, payload.to_date,
    );

    let result = trade_service::get_trades_range(
        &pool,
        payload.symbol_id,
        payload.from_date,
        payload.to_date,
    )
    .await;
    prepare_response(result)
}
