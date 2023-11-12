use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OhlcModel {
    bucket: DateTime<Utc>,
    open_price: BigDecimal,
    high: BigDecimal,
    low: BigDecimal,
    close_price: BigDecimal,
    volume: BigDecimal,
    symbol: String,
}

impl OhlcModel {
    pub fn new(
        bucket: DateTime<Utc>,
        open_price: BigDecimal,
        high: BigDecimal,
        low: BigDecimal,
        close_price: BigDecimal,
        volume: BigDecimal,
        symbol: String,
    ) -> OhlcModel {
        OhlcModel {
            bucket,
            open_price,
            high,
            low,
            close_price,
            volume,
            symbol,
        }
    }
}

impl OhlcModel {
    pub fn get_bucket(&self) -> &DateTime<Utc> {
        &self.bucket
    }
    pub fn get_open_price(&self) -> &BigDecimal {
        &self.open_price
    }
    pub fn get_high(&self) -> &BigDecimal {
        &self.high
    }
    pub fn get_low(&self) -> &BigDecimal {
        &self.low
    }
    pub fn get_close_price(&self) -> &BigDecimal {
        &self.close_price
    }
    pub fn get_volume(&self) -> &BigDecimal {
        &self.volume
    }
    pub fn get_symbol(&self) -> &str {
        &self.symbol[..]
    }
}

/*
impl OhlcModel {
    pub fn set_bucket(&mut self, time: DateTime<Utc>) {
        self.bucket = bucket;
    }

    pub fn set_price(&mut self, price: BigDecimal) {
        self.price = price;
    }

    pub fn set_volume(&mut self, volume: BigDecimal) {
        self.volume = volume;
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn set_order_type(&mut self, order_type: String) {
        self.order_type = order_type;
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }
}
 */

impl From<PgRow> for OhlcModel {
    fn from(value: PgRow) -> Self {
        OhlcModel::new(
            value.get("bucket"),
            value.get("open_price"),
            value.get("high"),
            value.get("low"),
            value.get("close_price"),
            value.get("volume"),
            value.get("symbol"),
        )
    }
}
