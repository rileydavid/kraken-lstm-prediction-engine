use std::collections::HashSet;

use crate::domain::convert::ConvertModel;
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Transaction};
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

use super::symbol_repository;

//TODO set path correctly 
const QUERY_COPY_CSV_FILE: &str = "COPY kraken_trade (time, price, volume, side, order_type, symbol_id) FROM '$1' DELIMITER ',';"; 

pub async fn copy_csv(
    tx: &mut Transaction<'static, Postgres>,
    file_path: String, 
) -> Result<String, GenericError> {
    match sqlx::query(QUERY_COPY_CSV_FILE)
        .bind(file_path)
        .execute(&mut *tx)
        .await
    {
        Ok(_data) => Ok("OK".into()),
        Err(err) => 
            Err(RepositoryError::general_error(err.to_string())),
    }
}

