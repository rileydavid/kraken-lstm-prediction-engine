use crate::domain::symbol::SymbolModel;
use sqlx::postgres::PgRow;
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

const QUERY_SELECT_SYMBOLS: &str = "SELECT * FROM symbol;";
const QUERY_INSERT_SYMBOL_RETURNING_ID_SYMBOL: &str =
    "SELECT out_id AS id, out_symbol AS symbol FROM insert_symbol($1);";
const QUERY_SELECT_SYMBOL_ID: &str = "SELECT * FROM symbol WHERE symbol = $1;";

pub async fn get_symbols(
    pool: &sqlx::PgPool,
) -> Result<Vec<SymbolModel>, GenericError> {
    match sqlx::query(QUERY_SELECT_SYMBOLS)
        .map(|row: PgRow| SymbolModel::from(row))
        .fetch_all(pool)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

pub async fn insert_symbol(
    pool: &sqlx::PgPool,
    symbol: String,
) -> Result<SymbolModel, GenericError> {
    match sqlx::query(QUERY_INSERT_SYMBOL_RETURNING_ID_SYMBOL)
        .bind(symbol)
        .map(|row: PgRow| SymbolModel::from(row))
        .fetch_one(pool)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

// get/add symbol id 
pub async fn add_symbol_id(
    pool: &sqlx::PgPool,
    symbol: &String,
) -> Result<i32, GenericError> {
    match insert_symbol(pool, symbol.into()).await {
        Ok(data) => Ok(data.get_id().to_owned()),
        Err(err) => Err(err),
    }
}

pub async fn get_symbol_id(pool: &sqlx::PgPool, symbol: &String) -> Result<i32, GenericError> {
    match sqlx::query(QUERY_SELECT_SYMBOL_ID)
        .bind(symbol)
        .map(|row: PgRow| SymbolModel::from(row))
        .fetch_optional(pool)
        .await
    {
        Ok(data) => Ok(
            match data.is_none() {
                true => 0,
                false => data.unwrap().get_id().to_owned(),
            }
        ),
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

