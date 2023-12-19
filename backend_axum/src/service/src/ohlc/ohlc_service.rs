use crate::ohlc::ohlc_dto::OhlcDto;
use chrono::DateTime;
use chrono::Utc;
use repository::repository::ohlc_repository;
use utils::core::postgresdb::Tx;
use utils::core::postgresdb::TxAsync;
use utils::error::generic_error::GenericError;

pub async fn get_ohlc_hour_symbol_interval(
    symbol_id: i32,
    interval: i32,
) -> Result<Vec<OhlcDto>, GenericError> {
    let mut tx = Tx::begin().await;
    match ohlc_repository::get_ohlc_hour_symbol_interval(&mut tx, symbol_id, interval).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<OhlcDto> = data
                .into_iter()
                .map(|model| OhlcDto::from(model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}

pub async fn get_ohlc_day_symbol_interval(
    symbol_id: i32,
    interval: i32,
) -> Result<Vec<OhlcDto>, GenericError> {
    let mut tx = Tx::begin().await;
    match ohlc_repository::get_ohlc_day_symbol_interval(&mut tx, symbol_id, interval).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<OhlcDto> = data
                .into_iter()
                .map(|model| OhlcDto::from(model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}

pub async fn get_ohlc_hour_start_date(
    symbol_id: i32,
    interval: i32,
    start_date: DateTime<Utc>,
) -> Result<Vec<OhlcDto>, GenericError> {
    let mut tx = Tx::begin().await;
    match ohlc_repository::get_ohlc_hour_start_date(&mut tx, symbol_id, interval, start_date).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<OhlcDto> = data
                .into_iter()
                .map(|model| OhlcDto::from(model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}

pub async fn get_ohlc_day_start_date(
    symbol_id: i32,
    interval: i32,
    start_date: DateTime<Utc>,
) -> Result<Vec<OhlcDto>, GenericError> {
    let mut tx = Tx::begin().await;
    match ohlc_repository::get_ohlc_day_start_date(&mut tx, symbol_id, interval, start_date).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<OhlcDto> = data
                .into_iter()
                .map(|model| OhlcDto::from(model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}
