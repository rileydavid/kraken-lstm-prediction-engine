use sqlx::{Postgres, Transaction};
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

pub async fn copy_csv(
    tx: &mut Transaction<'static, Postgres>,
    file_path: String, 
) -> Result<String, GenericError> {
    let prepared_query = format!("COPY kraken_trade (time, price, volume, side, order_type, symbol_id) FROM '{}' DELIMITER ',';", file_path);
    match sqlx::query(&prepared_query)
        .execute(&mut *tx)
        .await
    {
        Ok(_data) => {
            Ok("Ok".into())
        },
        Err(err) => 
            Err(RepositoryError::general_error(err.to_string())),
    }
}

