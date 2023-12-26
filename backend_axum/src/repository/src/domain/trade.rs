use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeModel {
    pub time: DateTime<Utc>,
    pub price: f64,
    pub volume: f64,
    pub side: String,
    pub order_type: String,
    pub symbol_id: i32,
}

impl TradeModel {
    pub fn new(
        time: DateTime<Utc>,
        price: f64,
        volume: f64,
        side: String,
        order_type: String,
        symbol_id: i32,
    ) -> TradeModel {
        TradeModel {
            time,
            price,
            volume,
            side,
            order_type,
            symbol_id,
        }
    }
}

impl TradeModel {
    pub fn get_time(&self) -> &DateTime<Utc> {
        &self.time
    }
    pub fn get_price(&self) -> &f64 {
        &self.price
    }
    pub fn get_volume(&self) -> &f64 {
        &self.volume
    }
    pub fn get_side(&self) -> &str {
        &self.side[..]
    }
    pub fn get_order_type(&self) -> &str {
        &self.order_type[..]
    }
    pub fn get_symbol_id(&self) -> &i32 {
        &self.symbol_id
    }
}

impl TradeModel {
    pub fn set_time(&mut self, time: DateTime<Utc>) {
        self.time = time;
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn set_order_type(&mut self, order_type: String) {
        self.order_type = order_type;
    }

    pub fn set_symbol_id(&mut self, symbol_id: i32) {
        self.symbol_id = symbol_id;
    }
}

impl From<PgRow> for TradeModel {
    fn from(value: PgRow) -> Self {
        TradeModel::new(
            value.get("time_"),
            value.get("price"),
            value.get("volume"),
            value.get("side"),
            value.get("order_type"),
            value.get("symbol_id"),
        )
    }
}
