use sqlx::{Postgres, Transaction};
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

//TODO set path correctly 
//const QUERY_COPY_CSV_FILE: &str = "COPY kraken_trade (time, price, volume, side, order_type, symbol_id) FROM '{}' DELIMITER ',';"; 

pub async fn copy_csv(
    tx: &mut Transaction<'static, Postgres>,
    file_path: String, 
) -> Result<String, GenericError> {

    let prepared_query = format!("COPY kraken_trade (time, price, volume, side, order_type, symbol_id) FROM '{}' DELIMITER ',';", file_path);

    match sqlx::query(&prepared_query)
       // .bind(prepared_query)
        .execute(&mut *tx)
        .await
    {
        Ok(_data) => Ok("OK".into()),
        Err(err) => 
            Err(RepositoryError::general_error(err.to_string())),
    }
}

