use chrono::{TimeZone, Utc};
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

const QUERY_REFRESH_OHLC_HOUR: &str = "CALL refresh_continuous_aggregate('ohlc_hour', $1, $2);"; // symbol

pub async fn copy_csv(pool: &sqlx::PgPool, file_path: String) -> Result<String, GenericError> {
    let prepared_query = format!(
        "COPY trade (time, price, volume, side, order_type, symbol_id) FROM '{}' DELIMITER ',';",
        file_path
    );
    match sqlx::query(&prepared_query).execute(pool).await {
        Ok(_data) => {
            match sqlx::query(QUERY_REFRESH_OHLC_HOUR)
                .bind(Utc.timestamp_opt(0, 0).unwrap()) // epoch
                .bind(Utc::now())
                .execute(pool)
                .await
            {
                Ok(_) => Ok("Ok".into()),
                Err(err) => Err(RepositoryError::general_error(err.to_string())),
            }
        }
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}
