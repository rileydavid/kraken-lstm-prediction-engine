use service::ohlc::ohlc_service;
//use utils::error::generic_error::GenericError;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::extract::Path;
use axum::Json;
use service::ohlc::ohlc_dto::OhlcDto;

pub async fn get_ohlc_hour_symbol_interval(
    Path(input): Path<(String, i32)>,
) -> Result<Json<Vec<OhlcDto>>, ResponseError> {
    let result = ohlc_service::get_ohlc_hour_symbol_interval(input.0, input.1).await;
    prepare_response(result)
}

pub async fn get_ohlc_day_symbol_interval(
    Path(input): Path<(String, i32)>,
) -> Result<Json<Vec<OhlcDto>>, ResponseError> {
    let result = ohlc_service::get_ohlc_day_symbol_interval(input.0, input.1).await;
    prepare_response(result)
}
