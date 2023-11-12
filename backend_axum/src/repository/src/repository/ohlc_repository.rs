use crate::domain::ohlc::OhlcModel;
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Transaction};
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

const QUERY_SELECT_GET_OHLC_HOUR_BY_SYMBOL_INTERVAL: &str = "SELECT * FROM get_ohlc_hour($1, $2)"; //interval, symbol
const QUERY_SELECT_GET_OHLC_DAY_BY_SYMBOL_INTERVAL: &str = "SELECT * FROM get_ohlc_day($1, $2)"; //interval, symbol

pub async fn get_ohlc_hour_symbol_interval(
    tx: &mut Transaction<'static, Postgres>,
    symbol: String,
    interval: i32,
) -> Result<Vec<OhlcModel>, GenericError> {
    // let connection = get_connection().await;
    match sqlx::query(QUERY_SELECT_GET_OHLC_HOUR_BY_SYMBOL_INTERVAL)
        .bind(interval)
        .bind(symbol)
        .map(|row: PgRow| OhlcModel::from(row))
        .fetch_all(&mut *tx)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

pub async fn get_ohlc_day_symbol_interval(
    tx: &mut Transaction<'static, Postgres>,
    symbol: String,
    interval: i32,
) -> Result<Vec<OhlcModel>, GenericError> {
    // let connection = get_connection().await;
    match sqlx::query(QUERY_SELECT_GET_OHLC_DAY_BY_SYMBOL_INTERVAL)
        .bind(interval)
        .bind(symbol)
        .map(|row: PgRow| OhlcModel::from(row))
        .fetch_all(&mut *tx)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}
