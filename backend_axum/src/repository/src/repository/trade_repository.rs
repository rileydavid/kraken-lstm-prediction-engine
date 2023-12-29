use crate::domain::trade::TradeModel;
use chrono::{DateTime, Utc};
use sqlx::postgres::{PgQueryResult, PgRow};
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

const QUERY_SELECT_GET_TRADES_RANGE: &str = "SELECT * FROM get_trades_range($1, $2, $3)"; //from_date, to_date, symbol_id

const QUERY_INSERT_TRADE: &str = "CALL insert_trade ($1, $2, $3, $4, $5, $6)";

pub async fn get_trades_range(
    pool: &sqlx::PgPool,
    symbol_id: i32,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
) -> Result<Vec<TradeModel>, GenericError> {
    // let connection = get_connection().await;
    match sqlx::query(QUERY_SELECT_GET_TRADES_RANGE)
        .bind(from_date)
        .bind(to_date)
        .bind(symbol_id)
        .map(|row: PgRow| TradeModel::from(row))
        .fetch_all(pool)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

pub async fn insert_trade(
    pool: &sqlx::PgPool,
    trade: TradeModel,
) -> Result<PgQueryResult, GenericError> {
    match sqlx::query(QUERY_INSERT_TRADE)
        .bind(trade.get_time())
        .bind(trade.get_price())
        .bind(trade.get_volume())
        .bind(trade.get_side())
        .bind(trade.get_order_type())
        .bind(trade.get_symbol_id())
        .execute(pool)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

pub async fn insert_trades(
    pool: &sqlx::PgPool,
    trades: Vec<TradeModel>,
) -> Result<PgQueryResult, GenericError> {
    let (times, prices, volumes, sides, order_types, symbol_ids): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = trades.into_iter().fold(
        (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        |(mut times, mut prices, mut volumes, mut sides, mut order_types, mut symbol_ids),
         trade| {
            times.push(trade.time);
            prices.push(trade.price);
            volumes.push(trade.volume);
            sides.push(trade.side);
            order_types.push(trade.order_type);
            symbol_ids.push(trade.symbol_id);
            (times, prices, volumes, sides, order_types, symbol_ids)
        },
    );

    match sqlx::query(
        "INSERT INTO trade (time, price, volume, side, order_type, symbol_id)
        SELECT * FROM UNNEST($1::timestamptz[], $2::float8[], $3::float8[], $4::varchar[], $5::varchar[], $6::int[])")
        .bind(&times[..])
        .bind(&prices[..])
        .bind(&volumes[..])
        .bind(&sides[..])
        .bind(&order_types[..])
        .bind(&symbol_ids[..])
    .execute(pool)
    .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}
