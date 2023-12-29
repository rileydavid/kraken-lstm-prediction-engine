use crate::domain::ohlc::OhlcModel;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

const QUERY_SELECT_GET_OHLC_HOUR_RANGE: &str = "SELECT * FROM get_ohlc_hour_range($1, $2, $3)"; //from_date, to_date, symbol_id
const QUERY_SELECT_GET_OHLC_HOUR_RANGE_REDUCED: &str = "SELECT bucket, symbol_id, close_price, volume, count FROM get_ohlc_hour_range($1, $2, $3)";
const QUERY_SELECT_GET_OHLC_DAY_RANGE: &str = "SELECT * FROM get_ohlc_day_range($1, $2, $3)"; //from_date, to_date, symbol_id



pub async fn get_ohlc_hour_range(
    pool: &sqlx::PgPool,
    symbol_id: i32,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
) -> Result<Vec<OhlcModel>, GenericError> {
    match sqlx::query(QUERY_SELECT_GET_OHLC_HOUR_RANGE)
        .bind(from_date)
        .bind(to_date)
        .bind(symbol_id)
        .map(|row: PgRow| OhlcModel::from(row))
        .fetch_all(pool)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

pub async fn get_ohlc_hour_range_reduced(
    pool: &sqlx::PgPool,
    symbol_id: i32,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
) -> Result<Vec<OhlcModel>, GenericError> {
    match sqlx::query(QUERY_SELECT_GET_OHLC_HOUR_RANGE_REDUCED)
        .bind(from_date)
        .bind(to_date)
        .bind(symbol_id)
        .map(|row: PgRow| OhlcModel::from(row))
        .fetch_all(pool)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}


pub async fn get_ohlc_day_range(
    pool: &sqlx::PgPool,
    symbol_id: i32,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
) -> Result<Vec<OhlcModel>, GenericError> {
    match sqlx::query(QUERY_SELECT_GET_OHLC_DAY_RANGE)
        .bind(from_date)
        .bind(to_date)
        .bind(symbol_id)
        .map(|row: PgRow| OhlcModel::from(row))
        .fetch_all(pool)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}