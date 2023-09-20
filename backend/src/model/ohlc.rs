use serde::{Serialize, Deserialize};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

#[derive(Serialize, sqlx::FromRow, Deserialize, Debug)]
pub struct Ohlc {
    //pub channel_id: i32,
    pub time: DateTime<Utc>,
    pub high: BigDecimal,
    pub open: BigDecimal,
    pub close: BigDecimal,
    pub low: BigDecimal,
    pub symbol: String,
}

impl Ohlc {
    pub fn new(time: DateTime<Utc>, high: BigDecimal, open: BigDecimal, close: BigDecimal, low: BigDecimal, symbol: String) -> Ohlc {
        Ohlc{
            time,
            high, 
            open,
            close,
            low,
            symbol
        }
    }
}