use crate::domain::symbol::SymbolModel;
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Transaction};
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

const QUERY_SELECT_SYMBOLS: &str = "SELECT symbol FROM symbols"; 

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

