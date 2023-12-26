use crate::domain::trade::TradeModel;
use sqlx::postgres::{PgQueryResult, PgRow};
use sqlx::{Postgres, Transaction};
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

const QUERY_SELECT_GET_TRADES_BY_SYMBOL_INTERVAL: &str = "SELECT * FROM get_trades($1, $2)"; //interval, symbol

const QUERY_INSERT_TRADE: &str = "CALL insert_trade ($1, $2, $3, $4, $5, $6)";

pub async fn get_trades_by_symbol_interval(
    tx: &mut Transaction<'static, Postgres>,
    symbol: String,
    interval: i32,
) -> Result<Vec<TradeModel>, GenericError> {
    // let connection = get_connection().await;
    match sqlx::query(QUERY_SELECT_GET_TRADES_BY_SYMBOL_INTERVAL)
        .bind(interval)
        .bind(symbol)
        .map(|row: PgRow| TradeModel::from(row))
        .fetch_all(&mut *tx)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

pub async fn insert_trade(
    tx: &mut Transaction<'static, Postgres>,
    trade: TradeModel,
) -> Result<PgQueryResult, GenericError> {
    match sqlx::query(QUERY_INSERT_TRADE)
        .bind(trade.get_time())
        .bind(trade.get_price())
        .bind(trade.get_volume())
        .bind(trade.get_side())
        .bind(trade.get_order_type())
        .bind(trade.get_symbol_id())
        .execute(&mut *tx)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

