use crate::domain::ohlc::OhlcModel;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Transaction};
use tracing::info;
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

const QUERY_SELECT_GET_OHLC_HOUR_BY_SYMBOL_INTERVAL: &str = "SELECT * FROM get_ohlc_hour($1, $2)"; //interval, symbol
const QUERY_SELECT_GET_OHLC_HOUR_BY_START_DATE: &str = "SELECT * FROM get_ohlc_hour_start_date($1, $2, $3)"; //interval, start_date, symbol

const QUERY_SELECT_GET_OHLC_DAY_BY_SYMBOL_INTERVAL: &str = "SELECT * FROM get_ohlc_day($1, $2)"; //interval, symbol
const QUERY_SELECT_GET_OHLC_DAY_BY_START_DATE: &str = "SELECT * FROM get_ohlc_day_start_date($1, $2, $3)"; //interval, start_date, symbol

const QUERY_SELECT_GET_OHLC_HOUR_RANGE: &str = "SELECT * FROM get_ohlc_hour_range($1, $2, $3)"; //from_date, to_date, symbol_id


pub async fn get_ohlc_hour_symbol_interval(
    tx: &mut Transaction<'static, Postgres>,
    symbol_id: i32,
    interval: i32,
) -> Result<Vec<OhlcModel>, GenericError> {

    match sqlx::query(QUERY_SELECT_GET_OHLC_HOUR_BY_SYMBOL_INTERVAL)
        .bind(interval)
        .bind(symbol_id)
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
    symbol_id: i32,
    interval: i32,
) -> Result<Vec<OhlcModel>, GenericError> {

    match sqlx::query(QUERY_SELECT_GET_OHLC_DAY_BY_SYMBOL_INTERVAL)
        .bind(interval)
        .bind(symbol_id)
        .map(|row: PgRow| OhlcModel::from(row))
        .fetch_all(&mut *tx)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

pub async fn get_ohlc_hour_start_date(
    tx: &mut Transaction<'static, Postgres>,
    symbol_id: i32,
    interval: i32,
    start_date: DateTime<Utc>
) -> Result<Vec<OhlcModel>, GenericError> {

    match sqlx::query(QUERY_SELECT_GET_OHLC_HOUR_BY_START_DATE)
        .bind(interval)
        .bind(start_date)
        .bind(symbol_id)
        .map(|row: PgRow| OhlcModel::from(row))
        .fetch_all(&mut *tx)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

pub async fn get_ohlc_day_start_date(
    tx: &mut Transaction<'static, Postgres>,
    symbol_id: i32,
    interval: i32,
    start_date: DateTime<Utc>
) -> Result<Vec<OhlcModel>, GenericError> {

    match sqlx::query(QUERY_SELECT_GET_OHLC_DAY_BY_START_DATE)
        .bind(interval)
        .bind(start_date)
        .bind(symbol_id)
        .map(|row: PgRow| OhlcModel::from(row))
        .fetch_all(&mut *tx)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

// ohlc_repository::get_ohlc_hour_range(&mut tx, symbol_id, interval, from_date, to_date).await {

pub async fn get_ohlc_hour_range(
    tx: &mut Transaction<'static, Postgres>,
    symbol_id: i32,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
) -> Result<Vec<OhlcModel>, GenericError> {
    info!("repository get_ohlc_hour_range");
    match sqlx::query(QUERY_SELECT_GET_OHLC_HOUR_RANGE)
        .bind(from_date)
        .bind(to_date)
        .bind(symbol_id)
        .map(|row: PgRow| OhlcModel::from(row))
        .fetch_all(&mut *tx)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}