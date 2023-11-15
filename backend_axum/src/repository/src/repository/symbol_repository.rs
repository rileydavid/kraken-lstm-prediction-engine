use crate::domain::symbol::SymbolModel;
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Transaction};
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

const QUERY_SELECT_SYMBOLS: &str = "SELECT id AS out_id, symbol AS out_symbol FROM symbols;";
const QUERY_INSERT_SYMBOL_RETURNING_ID_SYMBOL: &str =
    "SELECT * FROM insert_symbol($1);";

pub async fn get_symbols(
    tx: &mut Transaction<'static, Postgres>,
) -> Result<Vec<SymbolModel>, GenericError> {
    // let connection = get_connection().await;
    match sqlx::query(QUERY_SELECT_SYMBOLS)
        .map(|row: PgRow| SymbolModel::from(row))
        .fetch_all(&mut *tx)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

pub async fn insert_symbol(
    tx: &mut Transaction<'static, Postgres>,
    symbol: String,
) -> Result<SymbolModel, GenericError> {
    match sqlx::query(QUERY_INSERT_SYMBOL_RETURNING_ID_SYMBOL)
        .bind(symbol)
        .map(|row: PgRow| SymbolModel::from(row))
        .fetch_one(&mut *tx)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

pub async fn get_symbol_id(
    tx: &mut Transaction<'static, Postgres>,
    symbol: String,
) -> Result<i32, GenericError> {
    match insert_symbol(tx, symbol).await {
        Ok(data) => Ok(data.get_id().to_owned()),
        Err(err) => Err(err),
    }
}
