use crate::model::ohlc::Ohlc;
use crate::model::trade::Trade;

use chrono::{DateTime, Utc};
use log::error;
use log::info;
use sqlx::Error;
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use sqlx::Row;
use std::ops::Add;
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
        // improve this query --> currently takes quite some time to execute
        let rows = sqlx::query("SELECT DISTINCT symbol FROM kraken_trade")
            .fetch_all(&*self.pool)
            .await;

        info!("here");

        if rows.is_ok() {
            let mut symbols: Vec<String> = Vec::new();
            for row in rows.unwrap() {
                let symbol: String = row.get(0);
                symbols.push(symbol);
            }
            info!("{:?}", symbols.len());
            return Some(symbols);
        }
        return None;
    }

    // Todo: handle errors better here 
    /*
    pub async fn insert_trade_prediction(&self, trade: Trade) {
        let result = sqlx::query("insert into kraken_trade_prediction (time, price, volume, side, order_type, symbol) values ($1, $2, $3, $4, $5, $6)")
        .bind(&trade.time)
        .bind(sqlx::types::BigDecimal::from_str(&trade.price.to_string()).unwrap())
        .bind(sqlx::types::BigDecimal::from_str(&trade.volume.to_string()).unwrap())
        .bind(&trade.side)
        .bind(&trade.order_type)
        .bind(&trade.symbol)
        .execute(&*self.pool)
        .await;

        if result.is_err() {
            error!("Error whilst inserting Trade");
        }
    }
     */

    pub async fn insert_trades(&self, trades: Vec<Trade>) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
          
        for trade in trades {
            let _ = sqlx::query!("insert into kraken_trade (time, price, volume, side, order_type, symbol) values ($1, $2, $3, $4, $5, $6)", 
            &trade.time,
            sqlx::types::BigDecimal::from_str(&trade.price.to_string()).unwrap(),
            sqlx::types::BigDecimal::from_str(&trade.volume.to_string()).unwrap(),
            &trade.side,
            &trade.order_type,
            &trade.symbol).execute(transaction.as_mut()).await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    // Todo: handle errors better here 
    /*
    pub async fn insert_trade(&self, trade: &Trade) {
        // maybe there is a better way to convert the bigdecimal
        let result = sqlx::query("insert into kraken_trade (time, price, volume, side, order_type, symbol) values ($1, $2, $3, $4, $5, $6)")
        .bind(&trade.time)
        .bind(sqlx::types::BigDecimal::from_str(&trade.price.to_string()).unwrap())
        .bind(sqlx::types::BigDecimal::from_str(&trade.volume.to_string()).unwrap())
        .bind(&trade.side)
        .bind(&trade.order_type)
        .bind(&trade.symbol)
        .execute(&*self.pool)
        .await;

        if result.is_err() {
            error!("Error whilst inserting Trade");
        }
    }
     */

    // interval in minutes
    pub async fn get_trades(&self, interval: &str, symbol: &str) -> Option<Vec<Trade>> {
        //self.pool
        let query;

        match interval {
            "15" => {
                query = "SELECT * FROM kraken_trade WHERE time >= NOW() - INTERVAL '15 minutes' AND symbol = '".to_owned().add(symbol).add("'");
            }
            "30" => {
                query = "SELECT * FROM kraken_trade WHERE time >= NOW() - INTERVAL '30 minutes' AND symbol = '".to_owned().add(symbol).add("'");
            }
            "60" => {
                query = "SELECT * FROM kraken_trade WHERE time >= NOW() - INTERVAL '60 minutes' AND symbol = '".to_owned().add(symbol).add("'");
            }
            _ => {
                error!("Unknown interval {:?}", interval);
                return None;
            }
        }

        //originally used bind but that did not work
        let rows = sqlx::query(&query).fetch_all(&*self.pool).await;

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
            info!("{:?}", trades.len());
            return Some(trades);
        }
        return None;
    }

    // interval in minutes
    pub async fn get_ohlc(&self, interval: &str, symbol: &str) -> Option<Vec<Ohlc>> {
        //self.pool
        let query;

        match interval {
            "15" => {
                query = "SELECT * FROM one_min_candle WHERE bucket >= NOW() - INTERVAL '15 minutes' AND symbol = '".to_owned().add(symbol).add("'");
            }
            "30" => {
                query = "SELECT * FROM one_min_candle WHERE bucket >= NOW() - INTERVAL '30 minutes' AND symbol = '".to_owned().add(symbol).add("'");
            }
            "60" => {
                query = "SELECT * FROM one_min_candle WHERE bucket >= NOW() - INTERVAL '60 minutes' AND symbol = '".to_owned().add(symbol).add("'");
            }
            _ => {
                error!("Unknown interval {:?}", interval);
                return None;
            }
        }

        //originally used bind but that did not work
        let rows = sqlx::query(&query).fetch_all(&*self.pool).await;

        if rows.is_ok() {
            let mut ohlc_candles: Vec<Ohlc> = Vec::new();
            for row in rows.unwrap() {
                let time: DateTime<Utc> = row.get(0);
                let high: BigDecimal = row.get(1);
                let open: BigDecimal = row.get(2);
                let close: BigDecimal = row.get(3);
                let low: BigDecimal = row.get(4);
                let symbol: String = row.get(5);

                ohlc_candles.push(Ohlc::new(
                    time,
                    bigdecimal::BigDecimal::from_str(&high.to_string()).unwrap(),
                    bigdecimal::BigDecimal::from_str(&open.to_string()).unwrap(),
                    bigdecimal::BigDecimal::from_str(&close.to_string()).unwrap(),
                    bigdecimal::BigDecimal::from_str(&low.to_string()).unwrap(),
                    symbol,
                ));
            }
            info!("{:?}", ohlc_candles.len());
            return Some(ohlc_candles);
        }
        return None;
    }
}
