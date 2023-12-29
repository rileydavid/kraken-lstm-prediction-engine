use crate::trade::trade_dto::TradeDto;
use chrono::DateTime;
use chrono::Utc;
use repository::repository::trade_repository;
use utils::error::generic_error::GenericError;
use rayon::prelude::*;

pub async fn get_trades_range(
    pool: &sqlx::PgPool,
    symbol_id: i32,
    from_date: DateTime<Utc>,
    to_date: Option<DateTime<Utc>>,
) -> Result<Vec<TradeDto>, GenericError> {
    
    let adjusted_to_date = match to_date {
        Some(date) => date,
        None => Utc::now(),
    };

    match trade_repository::get_trades_range(pool, symbol_id, from_date, adjusted_to_date).await {
        Ok(data) => {
            let result: Vec<TradeDto> = data
                .into_par_iter()
                .map(|model| TradeDto::from(&model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}
