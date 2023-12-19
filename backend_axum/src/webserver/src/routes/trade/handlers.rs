use service::trade::trade_service;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::extract::Path;
use axum::Json;
use service::trade::trade_dto::TradeDto;

pub async fn get_trade_symbol_interval(
    Path(input): Path<(String, i32)>,
) -> Result<Json<Vec<TradeDto>>, ResponseError> {
    let result = trade_service::get_trades_by_symbol_interval(input.0, input.1).await;
    prepare_response(result)
}
