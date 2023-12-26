use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OhlcModel {
    bucket: DateTime<Utc>,
    open_price: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close_price: f64,
    volume: f64,
    count: i32,
    symbol_id: i32, 
}

impl OhlcModel {
    pub fn new(
        bucket: DateTime<Utc>,
        open_price: Option<f64>,
        high: Option<f64>,
        low: Option<f64>,
        close_price: f64,
        volume: f64,
        count: i32,
        symbol_id: i32,
    ) -> OhlcModel {
        OhlcModel {
            bucket,
            open_price,
            high,
            low,
            close_price,
            volume,
            count,
            symbol_id,
        }
    }
}

impl OhlcModel {
    pub fn get_bucket(&self) -> &DateTime<Utc> {
        &self.bucket
    }
    pub fn get_open_price(&self) -> &Option<f64> {
        &self.open_price
    }
    pub fn get_high(&self) -> &Option<f64> {
        &self.high
    }
    pub fn get_low(&self) -> &Option<f64> {
        &self.low
    }
    pub fn get_close_price(&self) -> &f64 {
        &self.close_price
    }
    pub fn get_volume(&self) -> &f64 {
        &self.volume
    }
    pub fn get_count(&self) -> &i32 {
        &self.count
    }
    pub fn get_symbol_id(&self) -> &i32 {
        &self.symbol_id
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
            value.try_get("open_price").ok(),
            value.try_get("high").ok(),
            value.try_get("low").ok(),
            value.get("close_price"),
            value.get("volume"),
            value.get("count"),
            value.get("symbol_id"),
        )
    }
}
