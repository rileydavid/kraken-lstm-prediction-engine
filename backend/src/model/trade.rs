use serde::{Serialize, Deserialize};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
//use strum_macros::{EnumString, Display};

#[derive(Serialize, sqlx::FromRow, Deserialize, Debug)]
pub struct Trade {
    //pub channel_id: i32,
    pub time: DateTime<Utc>,
    pub price: BigDecimal,
    pub volume: BigDecimal,
    pub side: String,
    pub order_type: String,
    pub symbol: String,
}

impl Trade {
    pub fn new(time: DateTime<Utc>, price: BigDecimal, volume: BigDecimal, side: String, order_type: String, symbol: String) -> Trade {
        Trade{
            time,
            price, 
            volume,
            side,
            order_type,
            symbol
        }
    }
}