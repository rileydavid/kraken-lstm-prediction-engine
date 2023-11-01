use crate::model::trade::Trade;
use log::error;
use log::info;
use log::warn;
use rayon::prelude::*;
use sqlx::Error;
use sqlx::PgPool;
use sqlx::Row;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone)]
pub struct PostgresRepository {
    pool: Arc<PgPool>,
}

impl PostgresRepository {
    pub fn init(pool: Arc<PgPool>) -> PostgresRepository {
        PostgresRepository { pool }
    }

    pub async fn get_symbols(&self) -> Option<Vec<String>> {
        let rows = sqlx::query("SELECT symbol FROM symbols")
            .fetch_all(&*self.pool)
            .await;

        match rows.is_ok() {
            true => Some(rows.unwrap().iter().map(|row| row.get(0)).collect()),
            false => None,
        }
    }

    pub async fn insert_trade(&self, trade: Trade) -> Result<(), Error> {
        //TODO: check result -- handle errors
        match sqlx::query!(
            "CALL insert_trade ($1, $2, $3, $4, $5, $6)",
            &trade.time,
            sqlx::types::BigDecimal::from_str(&trade.price.to_string()).unwrap(),
            sqlx::types::BigDecimal::from_str(&trade.volume.to_string()).unwrap(),
            &trade.side,
            &trade.order_type,
            &trade.symbol
        )
        .execute(&*self.pool)
        .await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Error occured during insert_trade: {}", err);
                Ok(()) // should actually retry somehow
            },
        }
    }

    // interval in minutes
    pub async fn get_trades(&self, interval: &str, symbol: &str) -> Option<Vec<Trade>> {
        info!("DB get_trades({} minutes, {})", interval, symbol);
        let timeframe = Self::validate_numeric(interval);
        let rows = sqlx::query!("SELECT * FROM get_trades($1, $2)", timeframe, symbol)
            .fetch_all(&*self.pool)
            .await;

        match rows.is_ok() {
            true => Some(
                rows.unwrap()
                    .par_iter()
                    .map(|row| {
                        Trade::new_from_sqlx_bigdecimal(
                            &row.time_.unwrap(),
                            row.price.as_ref().unwrap(),
                            row.volume.as_ref().unwrap(),
                            row.side.as_ref().unwrap(),
                            row.order_type.as_ref().unwrap(),
                            row.symbol.as_ref().unwrap(),
                        )
                    })
                    .collect::<Vec<Trade>>(),
            ),
            false => {
                error!("Error occured whilst fetching rows");
                None
            }
        }
    }

    /*
    pub async fn get_all_trades(&self, interval: &str) -> Option<Vec<Trade>> {
        info!("DB get_all_trades({} minutes)", interval);
        let timeframe = Self::validate_numeric(interval);
        let rows = sqlx::query!("SELECT * FROM get_all_trades($1)", timeframe)
            .fetch_all(&*self.pool)
            .await;

        match rows.is_ok() {
            true => Some(
                rows.unwrap()
                    .par_iter()
                    .map(|row| {
                        Trade::new_from_sqlx_bigdecimal(
                            &row.time_.unwrap(),
                            row.price.as_ref().unwrap(),
                            row.volume.as_ref().unwrap(),
                            row.side.as_ref().unwrap(),
                            row.order_type.as_ref().unwrap(),
                            row.symbol.as_ref().unwrap(),
                        )
                    })
                    .collect::<Vec<Trade>>(),
            ),
            false => {
                error!("Error occured whilst getting all trades");
                None
            }
        }
    }
 */
    /*
    pub async fn get_all_ohlc() -> Result<()> {
        Ok(1)
    }
     */


    fn validate_numeric(input: &str) -> i32 {
        let result = input.trim().parse::<i32>();
        match result {
            Ok(output) => output,
            Err(_) => {
                warn!("Invalid Timeframe provided using default value (15)");
                15
            }
        }
    }

    /*
        pub async fn insert_trades(&self, trades: Vec<Trade>) -> Result<(), Error> {
            let mut transaction = self.pool.begin().await?;

            for trade in trades {
                let _ = sqlx::query!(
                    "CALL insert_trade ($1, $2, $3, $4, $5, $6)",
                    &trade.time,
                    sqlx::types::BigDecimal::from_str(&trade.price.to_string()).unwrap(),
                    sqlx::types::BigDecimal::from_str(&trade.volume.to_string()).unwrap(),
                    &trade.side,
                    &trade.order_type,
                    &trade.symbol
                )
                .execute(transaction.as_mut())
                .await;
            }

            transaction.commit().await?;
            Ok(())
        }
    */
}
