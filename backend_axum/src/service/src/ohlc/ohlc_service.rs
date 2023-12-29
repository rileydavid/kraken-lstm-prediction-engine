use crate::ohlc::ohlc_dto::OhlcDto;
use chrono::DateTime;
use chrono::Utc;
use rayon::prelude::*;
use repository::repository::ohlc_repository;
use utils::error::generic_error::GenericError;

pub async fn get_ohlc_hour_range(
    pool: &sqlx::PgPool,
    symbol_id: i32,
    from_date: DateTime<Utc>,
    to_date: Option<DateTime<Utc>>,
) -> Result<Vec<OhlcDto>, GenericError> {
    let adjusted_to_date = match to_date {
        Some(date) => date,
        None => Utc::now(),
    };

    match ohlc_repository::get_ohlc_hour_range(pool, symbol_id, from_date, adjusted_to_date)
        .await
    {
        Ok(data) => {
            let result: Vec<OhlcDto> = data
                .into_par_iter()
                .map(|model| OhlcDto::from(&model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}

pub async fn get_ohlc_day_range(
    pool: &sqlx::PgPool,
    symbol_id: i32,
    from_date: DateTime<Utc>,
    to_date: Option<DateTime<Utc>>,
) -> Result<Vec<OhlcDto>, GenericError> {
    let adjusted_to_date = match to_date {
        Some(date) => date,
        None => Utc::now(),
    };

    match ohlc_repository::get_ohlc_day_range(pool, symbol_id, from_date, adjusted_to_date)
        .await
    {
        Ok(data) => {
            let result: Vec<OhlcDto> = data
                .into_par_iter()
                .map(|model| OhlcDto::from(&model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}
