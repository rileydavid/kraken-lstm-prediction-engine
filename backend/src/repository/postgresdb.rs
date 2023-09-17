use crate::model::trade::Trade;
use chrono::{DateTime, Utc};
use log::error;
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use sqlx::Row;
use std::str::FromStr;

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn init(pool: PgPool) -> PostgresRepository {
        PostgresRepository { pool }
    }

    // interval in minutes
    pub async fn get_trades(&self, interval: &str) -> Option<Vec<Trade>> {
        //self.pool
        let interval_value;

        match interval {
            "15" => {
                interval_value = 15;
            }
            "30" => {
                interval_value = 30;
            }
            "60" => {
                interval_value = 60;
            }
            _ => {
                error!("Unknown interval {:?}", interval);
                return None;
            }
        }

        let rows = sqlx::query("SELECT * FROM kraken_trade WHERE time >= NOW() - INTERVAL '$1 minutes'")
            .bind(interval_value)
            .fetch_all(&self.pool)
            .await;

        if rows.is_ok() {
            let mut trades: Vec<Trade> = Vec::new();
            for row in rows.unwrap() {
                let time: DateTime<Utc> = row.get(0);
                let price: BigDecimal = row.get(1);
                let volume: BigDecimal = row.get(2);
                let side: String = row.get(3);
                let order_type = row.get(4);
                let symbol: String = row.get(5);

                trades.push(Trade::new(
                    time,
                    bigdecimal::BigDecimal::from_str(&price.to_string()).unwrap(),
                    bigdecimal::BigDecimal::from_str(&volume.to_string()).unwrap(),
                    side,
                    order_type,
                    symbol,
                ));
            }
            return Some(trades);
        }
        return None;
    }
}
